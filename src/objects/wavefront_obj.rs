use std::{error::Error, fs::read_to_string, path::Path};

use crate::{
    linalg::{Vec3, Vec4},
    material::Material,
    objects::{IntoPrimitives, Object, Triangle},
};

#[derive(Default)]
pub struct WavefrontObj {
    vertices: Vec<Vec4>,
    texture_coords: Vec<Vec3>, // TODO: Implement reading this
    vertex_normals: Vec<Vec3>,
    parameter_vertices: Vec<Vec3>, // TODO: Implement reading this
    faces: Vec<Vec<ObjIndex>>,
    lines: Vec<Vec<usize>>,
}

impl WavefrontObj {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, WavefrontObjError> {
        // TODO: Implement support for negative indices
        let data = read_to_string(path)?;
        let mut obj = Self::default();
        for (row_num, row) in data.split("\n").enumerate() {
            let row_num = row_num + 1;
            if row.starts_with("#") || row.is_empty() {
                continue;
            } else if row.starts_with("v ") {
                let coords: Vec<_> = row.split(" ").skip(1).map(|x| x.parse::<f64>()).collect();
                if coords.iter().any(|c| c.is_err()) {
                    return Err(WavefrontObjError::ParseError(row_num));
                }
                if coords.len() < 3 {
                    return Err(WavefrontObjError::ParseError(row_num));
                }
                obj.vertices.push(Vec4::new(
                    coords[0].clone().unwrap(),
                    coords[1].clone().unwrap(),
                    coords[2].clone().unwrap(),
                    if coords.len() >= 4 {
                        coords[3].clone().unwrap()
                    } else {
                        1.0
                    },
                ));
            } else if row.starts_with("f ") {
                let elems: Vec<String> = row.split(" ").skip(1).map(|s| s.to_owned()).collect();
                if elems.len() < 3 {
                    return Err(WavefrontObjError::IncompleteFace(row_num));
                }
                let mut face = Vec::new();
                for el in elems {
                    let mut index = ObjIndex::default();
                    let parts: Vec<_> = el.split("/").collect();
                    if parts.len() > 3 {
                        return Err(WavefrontObjError::ParseError(row_num));
                    }
                    if !parts.is_empty() {
                        let idx = parts[0].parse::<usize>();
                        if idx.is_err() {
                            return Err(WavefrontObjError::ParseError(row_num));
                        }
                        index.vertex = idx.unwrap() - 1;
                    }
                    if parts.len() >= 2 && !parts[1].is_empty() {
                        let idx = parts[1].parse::<usize>();
                        if idx.is_err() {
                            return Err(WavefrontObjError::ParseError(row_num));
                        }
                        index.texture = Some(idx.unwrap() - 1);
                    }
                    if parts.len() >= 3 && !parts[2].is_empty() {
                        let idx = parts[2].parse::<usize>();
                        if idx.is_err() {
                            return Err(WavefrontObjError::ParseError(row_num));
                        }
                        index.normal = Some(idx.unwrap() - 1);
                    }
                    face.push(index);
                }
                obj.faces.push(face);
            } else if row.starts_with("vn ") {
                let coords: Vec<_> = row.split(" ").skip(1).map(|x| x.parse::<f64>()).collect();
                if coords.iter().any(|c| c.is_err()) {
                    return Err(WavefrontObjError::ParseError(row_num));
                }
                if coords.len() != 3 {
                    return Err(WavefrontObjError::ParseError(row_num));
                }
                obj.vertex_normals.push(Vec3::new(
                    coords[0].clone().unwrap(),
                    coords[1].clone().unwrap(),
                    coords[2].clone().unwrap(),
                ));
            } else if row.starts_with("l ") {
                let idxs: Vec<_> = row.split(" ").skip(1).map(|x| x.parse::<usize>()).collect();
                if idxs.iter().any(|c| c.is_err()) {
                    return Err(WavefrontObjError::ParseError(row_num));
                }
                obj.lines
                    .push(idxs.iter().map(|j| j.clone().unwrap() - 1).collect());
            } else {
                return Err(WavefrontObjError::UnsupportedType(row.to_owned(), row_num));
            }
        }
        Ok(obj)
    }

    pub fn triangulate(&self, material: Material) -> Surface {
        let mut surface = Surface(vec![]);
        for face in self.faces.iter() {
            let idx0 = face[0].clone();
            let p0 = self.vertices[idx0.vertex].pr3();
            for window in face[1..].windows(2) {
                let idx1 = window[0].clone();
                let idx2 = window[1].clone();
                let p1 = self.vertices[idx1.vertex].pr3();
                let p2 = self.vertices[idx2.vertex].pr3();
                let triangle = Triangle::new(p0, p1 - p0, p2 - p0, material.clone());
                surface.0.push(triangle);
            }
        }
        surface
    }
}

#[derive(Default, Debug, Clone)]
struct ObjIndex {
    vertex: usize,
    texture: Option<usize>,
    normal: Option<usize>,
}

#[derive(Debug)]
pub enum WavefrontObjError {
    IOError(std::io::Error),
    UnsupportedType(String, usize),
    // TODO: Forward the various kinds of parse errors here to generate more helpful error messages
    ParseError(usize),
    IncompleteFace(usize),
}

impl std::fmt::Display for WavefrontObjError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::IOError(ref err) => write!(f, "{err}"),
            Self::UnsupportedType(ref typ, line) => {
                write!(f, "Encountered unsupported type {typ} in line {line}")
            }
            Self::IncompleteFace(line) => {
                write!(f, "Not enough vertices to form a face in line {line}")
            }
            Self::ParseError(line) => write!(f, "Error parsing line {line}"),
        }
    }
}

impl Error for WavefrontObjError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            Self::IOError(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for WavefrontObjError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

pub struct Surface(Vec<Object>);

impl IntoPrimitives for Surface {
    fn primitives(&self) -> Vec<super::Object> {
        self.0.to_vec()
    }
}

