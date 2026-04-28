//! Floorplan → DXF synthesis. The heart of Layer 3 → Layer 1.

use std::collections::HashMap;
use std::io::Write;

use cad_core::error::Result;
use cad_core::{BlockDef, Entity, EntityProps, LayerName, Point};
use cad_dxf_writer::layer::standard_arch_layers;
use cad_dxf_writer::write_dxf;
use cad_semantic::{Floorplan, Opening, OpeningKind, Room, Wall};

use crate::blocks;

#[derive(Debug, Clone)]
pub struct SynthesizeConfig {
    pub wall_layer: String,
    pub door_layer: String,
    pub window_layer: String,
    pub dimension_layer: String,
    pub room_label_layer: String,
    pub grid_layer: String,
    pub room_label_height: f64,
    pub grid_label_height: f64,
    pub include_grid: bool,
    pub include_dimensions: bool,
    pub include_room_labels: bool,
    pub acad_version: String,
    pub units_code: String,
}

impl Default for SynthesizeConfig {
    fn default() -> Self {
        Self {
            wall_layer: "WALLS".into(),
            door_layer: "DOORS".into(),
            window_layer: "WINDOWS".into(),
            dimension_layer: "DIMENSIONS".into(),
            room_label_layer: "TEXT".into(),
            grid_layer: "GRID".into(),
            room_label_height: 250.0,
            grid_label_height: 160.0,
            include_grid: true,
            include_dimensions: true,
            include_room_labels: true,
            acad_version: "AC1015".into(),
            units_code: "4".into(), // millimeters
        }
    }
}

/// Synthesize a [`Floorplan`] to a DXF stream.
pub fn floorplan_to_dxf<W: Write>(
    plan: &Floorplan,
    writer: W,
    config: &SynthesizeConfig,
) -> Result<()> {
    let walls_by_id: HashMap<u32, &Wall> =
        plan.walls.iter().map(|w| (w.id, w)).collect();

    let mut entities: Vec<Entity> = vec![];

    // Walls → 2 parallel LINEs each. Preserve each wall's own layer name
    // (so Korean/English layer-hint classification round-trips); fall back to
    // the config default only when the wall has no meaningful layer.
    for w in &plan.walls {
        let layer = if w.layer.is_empty() || w.layer == "0" {
            config.wall_layer.as_str()
        } else {
            w.layer.as_str()
        };
        let (a, b) = wall_to_parallel_pair(w, layer);
        entities.push(a);
        entities.push(b);
    }

    // Openings → INSERT references.
    let mut needed_blocks: HashMap<String, BlockDef> = HashMap::new();
    for o in &plan.openings {
        let host = match o.host_wall.and_then(|id| walls_by_id.get(&id).copied()) {
            Some(h) => h,
            None => continue,
        };
        let (insert, block) = opening_to_insert(o, host, config);
        entities.push(insert);
        needed_blocks.entry(block.name.clone()).or_insert(block);
    }

    // Room labels → TEXT.
    if config.include_room_labels {
        for r in &plan.rooms {
            if let Some(label) = &r.label {
                entities.push(room_label_to_text(r, label, config));
            }
        }
    }

    // Dimensions → TEXT (LINE+TEXT pattern simplified to just TEXT).
    if config.include_dimensions {
        for d in &plan.dimensions {
            entities.push(dimension_to_text(d, config));
        }
    }

    // Grid → LINEs + label TEXTs.
    if config.include_grid {
        for g in &plan.grids {
            let mut grid_entities = grid_to_entities(g, config);
            entities.append(&mut grid_entities);
        }
    }

    // Build the final DXF.
    let layers = standard_arch_layers();
    let header_vars: Vec<(&str, &str)> = vec![
        ("$ACADVER", config.acad_version.as_str()),
        ("$INSUNITS", config.units_code.as_str()),
    ];
    let block_vec: Vec<BlockDef> = needed_blocks.into_values().collect();

    write_dxf(writer, &header_vars, &layers, &block_vec, &entities)?;
    Ok(())
}

fn wall_to_parallel_pair(w: &Wall, layer: &str) -> (Entity, Entity) {
    let v = &w.centerline.vertices;
    if v.len() < 2 {
        // Degenerate — emit two zero-length LINEs (caller can check).
        let z = Point::new(0.0, 0.0, 0.0);
        return (
            line_entity(z, z, layer),
            line_entity(z, z, layer),
        );
    }
    let a = v[0];
    let b = v[v.len() - 1];
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    let len = dx.hypot(dy);
    if len < 1e-9 {
        let z = Point::new(0.0, 0.0, 0.0);
        return (line_entity(z, z, layer), line_entity(z, z, layer));
    }
    let nx = -dy / len;
    let ny = dx / len;
    let half = w.thickness / 2.0;
    let a_plus = Point::new(a.x + nx * half, a.y + ny * half, 0.0);
    let b_plus = Point::new(b.x + nx * half, b.y + ny * half, 0.0);
    let a_minus = Point::new(a.x - nx * half, a.y - ny * half, 0.0);
    let b_minus = Point::new(b.x - nx * half, b.y - ny * half, 0.0);
    (
        line_entity(a_plus, b_plus, layer),
        line_entity(a_minus, b_minus, layer),
    )
}

fn opening_to_insert(
    o: &Opening,
    host: &Wall,
    config: &SynthesizeConfig,
) -> (Entity, BlockDef) {
    let v = &host.centerline.vertices;
    let (a, b) = (v[0], v[v.len() - 1]);
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    let len = dx.hypot(dy);
    let dir_x = if len > 1e-9 { dx / len } else { 1.0 };
    let dir_y = if len > 1e-9 { dy / len } else { 0.0 };

    // Hinge / center position along centerline.
    let hinge = Point::new(
        a.x + dir_x * o.position_along_wall,
        a.y + dir_y * o.position_along_wall,
        0.0,
    );

    let wall_angle = dir_y.atan2(dir_x);

    let (block, rotation, layer) = match o.kind {
        OpeningKind::Door => {
            let (std_w, name) = blocks::nearest_door_block(o.width);
            let block = blocks::standard_door_block(std_w);
            // Door leaf is perpendicular to wall, swinging along wall direction.
            let rotation = wall_angle - std::f64::consts::FRAC_PI_2;
            (block_with_name(block, name), rotation, config.door_layer.clone())
        }
        OpeningKind::Window | OpeningKind::Pass => {
            let (std_w, name) = blocks::nearest_window_block(o.width);
            let block = blocks::standard_window_block(std_w, blocks::DEFAULT_WINDOW_DEPTH_MM);
            // Window aligned with wall direction.
            let rotation = wall_angle;
            (block_with_name(block, name), rotation, config.window_layer.clone())
        }
    };

    let std_width = parse_block_width(&block.name);
    let scale = if std_width > 1e-6 {
        o.width / std_width
    } else {
        1.0
    };

    (
        Entity::Insert {
            block_name: block.name.clone(),
            position: hinge,
            scale_x: scale,
            scale_y: scale,
            scale_z: 1.0,
            rotation,
            layer,
            props: EntityProps::default(),
        },
        block,
    )
}

fn block_with_name(mut b: BlockDef, name: String) -> BlockDef {
    b.name = name;
    b
}

fn parse_block_width(block_name: &str) -> f64 {
    let suffix: String = block_name
        .chars()
        .rev()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    let suffix: String = suffix.chars().rev().collect();
    suffix.parse::<f64>().unwrap_or(0.0)
}

fn room_label_to_text(r: &Room, label: &str, config: &SynthesizeConfig) -> Entity {
    let centroid = r.boundary.centroid();
    Entity::Text {
        position: centroid,
        value: label.into(),
        height: config.room_label_height,
        rotation: 0.0,
        layer: config.room_label_layer.clone(),
        props: EntityProps::default(),
    }
}

fn dimension_to_text(
    d: &cad_semantic::Dimension,
    config: &SynthesizeConfig,
) -> Entity {
    let mid = Point::new(
        (d.origin.x + d.target.x) / 2.0,
        (d.origin.y + d.target.y) / 2.0,
        0.0,
    );
    let value = d
        .text_override
        .clone()
        .unwrap_or_else(|| format!("{:.0}", d.measured_value));
    Entity::Text {
        position: mid,
        value,
        height: 180.0,
        rotation: 0.0,
        layer: config.dimension_layer.clone(),
        props: EntityProps::default(),
    }
}

fn grid_to_entities(g: &cad_semantic::Grid, config: &SynthesizeConfig) -> Vec<Entity> {
    let mut out: Vec<Entity> = vec![];
    for axis in &g.x_axes {
        out.push(line_entity(axis.start, axis.end, &config.grid_layer));
        if let Some(label) = &axis.label {
            // Place label at the lower (smaller-Y) endpoint, slightly offset down.
            let label_pos = Point::new(
                axis.start.x.min(axis.end.x).min(axis.position) - 80.0,
                axis.start.y.min(axis.end.y) - 400.0,
                0.0,
            );
            out.push(text_entity(
                label_pos,
                label,
                config.grid_label_height,
                &config.grid_layer,
            ));
        }
    }
    for axis in &g.y_axes {
        out.push(line_entity(axis.start, axis.end, &config.grid_layer));
        if let Some(label) = &axis.label {
            let label_pos = Point::new(
                axis.start.x.min(axis.end.x) - 900.0,
                axis.position - 80.0,
                0.0,
            );
            out.push(text_entity(
                label_pos,
                label,
                config.grid_label_height,
                &config.grid_layer,
            ));
        }
    }
    out
}

fn line_entity(p1: Point, p2: Point, layer: &str) -> Entity {
    Entity::Line { p1, p2, layer: layer.into(), props: EntityProps::default() }
}

fn text_entity(pos: Point, value: &str, height: f64, layer: &str) -> Entity {
    Entity::Text {
        position: pos,
        value: value.into(),
        height,
        rotation: 0.0,
        layer: layer.into(),
        props: EntityProps::default(),
    }
}

#[allow(dead_code)]
fn _force_layer_name_used(_: LayerName) {}
