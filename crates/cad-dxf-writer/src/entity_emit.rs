//! Entity-specific DXF emission. Per the `AutoCAD` DXF spec each entity has a
//! fixed group-code recipe; this module encodes those recipes.

use std::io::Write;

use cad_core::error::Result;
use cad_core::Entity;

use crate::{format_f, write_pair_to};

pub fn emit_entity<W: Write>(w: &mut W, e: &Entity) -> Result<()> {
    match e {
        Entity::Line {
            p1,
            p2,
            layer,
            props,
        } => {
            write_pair_to(w, 0, "LINE")?;
            write_pair_to(w, 8, layer)?;
            write_pair_to(w, 100, "AcDbEntity")?;
            write_pair_to(w, 100, "AcDbLine")?;
            write_pair_to(w, 10, &format_f(p1.x))?;
            write_pair_to(w, 20, &format_f(p1.y))?;
            write_pair_to(w, 30, &format_f(p1.z))?;
            write_pair_to(w, 11, &format_f(p2.x))?;
            write_pair_to(w, 21, &format_f(p2.y))?;
            write_pair_to(w, 31, &format_f(p2.z))?;
            emit_extras(w, props)?;
        }
        Entity::Circle {
            center,
            radius,
            layer,
            props,
        } => {
            write_pair_to(w, 0, "CIRCLE")?;
            write_pair_to(w, 8, layer)?;
            write_pair_to(w, 100, "AcDbEntity")?;
            write_pair_to(w, 100, "AcDbCircle")?;
            write_pair_to(w, 10, &format_f(center.x))?;
            write_pair_to(w, 20, &format_f(center.y))?;
            write_pair_to(w, 30, &format_f(center.z))?;
            write_pair_to(w, 40, &format_f(*radius))?;
            emit_extras(w, props)?;
        }
        Entity::Arc {
            center,
            radius,
            start_angle,
            end_angle,
            layer,
            props,
        } => {
            write_pair_to(w, 0, "ARC")?;
            write_pair_to(w, 8, layer)?;
            write_pair_to(w, 100, "AcDbEntity")?;
            write_pair_to(w, 100, "AcDbCircle")?;
            write_pair_to(w, 10, &format_f(center.x))?;
            write_pair_to(w, 20, &format_f(center.y))?;
            write_pair_to(w, 30, &format_f(center.z))?;
            write_pair_to(w, 40, &format_f(*radius))?;
            write_pair_to(w, 100, "AcDbArc")?;
            write_pair_to(w, 50, &format_f(start_angle.to_degrees()))?;
            write_pair_to(w, 51, &format_f(end_angle.to_degrees()))?;
            emit_extras(w, props)?;
        }
        Entity::LwPolyline {
            vertices,
            bulges,
            closed,
            layer,
            props,
        } => {
            write_pair_to(w, 0, "LWPOLYLINE")?;
            write_pair_to(w, 8, layer)?;
            write_pair_to(w, 100, "AcDbEntity")?;
            write_pair_to(w, 100, "AcDbPolyline")?;
            write_pair_to(w, 90, &format!("{}", vertices.len()))?;
            write_pair_to(w, 70, if *closed { "1" } else { "0" })?;
            for (i, v) in vertices.iter().enumerate() {
                write_pair_to(w, 10, &format_f(v.x))?;
                write_pair_to(w, 20, &format_f(v.y))?;
                let bulge = bulges.get(i).copied().unwrap_or(0.0);
                if bulge.abs() > 1e-9 {
                    write_pair_to(w, 42, &format_f(bulge))?;
                }
            }
            emit_extras(w, props)?;
        }
        Entity::Text {
            position,
            value,
            height,
            rotation,
            layer,
            props,
        } => {
            write_pair_to(w, 0, "TEXT")?;
            write_pair_to(w, 8, layer)?;
            write_pair_to(w, 100, "AcDbEntity")?;
            write_pair_to(w, 100, "AcDbText")?;
            write_pair_to(w, 10, &format_f(position.x))?;
            write_pair_to(w, 20, &format_f(position.y))?;
            write_pair_to(w, 30, &format_f(position.z))?;
            write_pair_to(w, 40, &format_f(*height))?;
            write_pair_to(w, 1, value)?;
            if rotation.abs() > 1e-9 {
                write_pair_to(w, 50, &format_f(rotation.to_degrees()))?;
            }
            write_pair_to(w, 7, "STANDARD")?;
            emit_extras(w, props)?;
        }
        Entity::MText {
            position,
            value,
            height,
            rotation,
            attachment_point,
            layer,
            props,
        } => {
            write_pair_to(w, 0, "MTEXT")?;
            write_pair_to(w, 8, layer)?;
            write_pair_to(w, 100, "AcDbEntity")?;
            write_pair_to(w, 100, "AcDbMText")?;
            write_pair_to(w, 10, &format_f(position.x))?;
            write_pair_to(w, 20, &format_f(position.y))?;
            write_pair_to(w, 30, &format_f(position.z))?;
            write_pair_to(w, 40, &format_f(*height))?;
            write_pair_to(w, 71, &format!("{attachment_point}"))?;
            write_pair_to(w, 1, value)?;
            if rotation.abs() > 1e-9 {
                write_pair_to(w, 50, &format_f(rotation.to_degrees()))?;
            }
            emit_extras(w, props)?;
        }
        Entity::Insert {
            block_name,
            position,
            scale_x,
            scale_y,
            scale_z,
            rotation,
            layer,
            props,
        } => {
            write_pair_to(w, 0, "INSERT")?;
            write_pair_to(w, 8, layer)?;
            write_pair_to(w, 100, "AcDbEntity")?;
            write_pair_to(w, 100, "AcDbBlockReference")?;
            write_pair_to(w, 2, block_name)?;
            write_pair_to(w, 10, &format_f(position.x))?;
            write_pair_to(w, 20, &format_f(position.y))?;
            write_pair_to(w, 30, &format_f(position.z))?;
            write_pair_to(w, 41, &format_f(*scale_x))?;
            write_pair_to(w, 42, &format_f(*scale_y))?;
            write_pair_to(w, 43, &format_f(*scale_z))?;
            if rotation.abs() > 1e-9 {
                write_pair_to(w, 50, &format_f(rotation.to_degrees()))?;
            }
            emit_extras(w, props)?;
        }
        Entity::Dimension {
            kind,
            block_name,
            text_override,
            measured_value,
            defining_points,
            rotation,
            layer,
            props,
        } => {
            write_pair_to(w, 0, "DIMENSION")?;
            write_pair_to(w, 8, layer)?;
            write_pair_to(w, 100, "AcDbEntity")?;
            if let Some(b) = block_name {
                write_pair_to(w, 2, b)?;
            }
            for (i, p) in defining_points.iter().enumerate() {
                let code_x = (10 + i) as i32;
                let code_y = (20 + i) as i32;
                let code_z = (30 + i) as i32;
                write_pair_to(w, code_x, &format_f(p.x))?;
                write_pair_to(w, code_y, &format_f(p.y))?;
                write_pair_to(w, code_z, &format_f(p.z))?;
            }
            write_pair_to(w, 70, &format!("{}", kind.0))?;
            if let Some(t) = text_override {
                write_pair_to(w, 1, t)?;
            }
            if let Some(m) = measured_value {
                write_pair_to(w, 42, &format_f(*m))?;
            }
            if rotation.abs() > 1e-9 {
                write_pair_to(w, 50, &format_f(rotation.to_degrees()))?;
            }
            emit_extras(w, props)?;
        }
        Entity::Raw(raw) => {
            write_pair_to(w, 0, &raw.kind)?;
            for (code, value) in &raw.groups {
                write_pair_to(w, *code, value)?;
            }
        }
    }
    Ok(())
}

fn emit_extras<W: Write>(w: &mut W, props: &cad_core::EntityProps) -> Result<()> {
    if let Some(c) = props.color {
        write_pair_to(w, 62, &format!("{c}"))?;
    }
    if let Some(t) = &props.linetype {
        write_pair_to(w, 6, t)?;
    }
    // Preserve unrecognized group codes captured by the parser. They must be
    // re-emitted to keep round-trip equivalence; if dropped, TEXT alignment,
    // subclass-specific fields, handle-like metadata etc. would silently
    // change after a parse→write cycle.
    for (code, value) in &props.raw_extras {
        write_pair_to(w, *code, value)?;
    }
    for (code, value) in &props.xdata {
        write_pair_to(w, *code, value)?;
    }
    Ok(())
}
