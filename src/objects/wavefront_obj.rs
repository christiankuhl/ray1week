use std::{error::Error, fs::read_to_string, path::Path};

use crate::{
    linalg::{Vec3, Vec4},
    material::Material,
    objects::{IntoPrimitives, Object, Triangle},
};

#[derive(Default)]
pub struct WavefrontObj {
    file: String,
    vertices: Vec<Vec4>,
    texture_coords: Vec<Vec3>,
    vertex_normals: Vec<Vec3>,
    parameter_vertices: Vec<OptParams>,
    faces: Vec<(usize, Vec<ObjIndex>)>,
    lines: Vec<(usize, Vec<isize>)>,
}

impl WavefrontObj {
    #[allow(clippy::field_reassign_with_default)]
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, WavefrontObjError> {
        let data = read_to_string(&path)?;
        let mut obj = Self::default();
        obj.file = path
            .as_ref()
            .as_os_str()
            .to_owned()
            .into_string()
            .expect("You have done something so esoteric, you probably deserve this.");
        for (row_num, row) in data.split("\n").enumerate() {
            let row_num = row_num + 1;
            if row.starts_with("#") || row.is_empty() {
                continue;
            } else if row.starts_with("v ") {
                let coords: Vec<_> = row.split(" ").skip(1).map(|x| x.parse::<f64>()).collect();
                if let Some(c) = coords.iter().find(|c| c.is_err()) {
                    let err = Box::new(c.clone().err().unwrap());
                    return Err(WavefrontObjError::ParseError(obj.file, row_num, err));
                }
                if coords.len() < 3 {
                    return Err(WavefrontObjError::IncompleteVertex(obj.file, row_num));
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
                    return Err(WavefrontObjError::IncompleteFace(obj.file, row_num));
                }
                let mut face = Vec::new();
                for el in elems {
                    let mut index = ObjIndex::default();
                    let parts: Vec<_> = el.split("/").collect();
                    if parts.len() > 3 {
                        let err = Box::new(WavefrontObjError::MalformedIndex);
                        return Err(WavefrontObjError::ParseError(obj.file, row_num, err));
                    }
                    if !parts.is_empty() {
                        let idx = parts[0].parse::<isize>();
                        if idx.is_err() {
                            let err = Box::new(idx.err().unwrap());
                            return Err(WavefrontObjError::ParseError(obj.file, row_num, err));
                        }
                        index.vertex = idx.unwrap();
                    }
                    if parts.len() >= 2 && !parts[1].is_empty() {
                        let idx = parts[1].parse::<isize>();
                        if idx.is_err() {
                            let err = Box::new(idx.err().unwrap());
                            return Err(WavefrontObjError::ParseError(obj.file, row_num, err));
                        }
                        index.texture = Some(idx.unwrap());
                    }
                    if parts.len() >= 3 && !parts[2].is_empty() {
                        let idx = parts[2].parse::<isize>();
                        if idx.is_err() {
                            let err = Box::new(idx.err().unwrap());
                            return Err(WavefrontObjError::ParseError(obj.file, row_num, err));
                        }
                        index.normal = Some(idx.unwrap());
                    }
                    face.push(index);
                }
                obj.faces.push((row_num, face));
            } else if row.starts_with("vn ") {
                let coords: Vec<_> = row.split(" ").skip(1).map(|x| x.parse::<f64>()).collect();
                if let Some(c) = coords.iter().find(|c| c.is_err()) {
                    let err = Box::new(c.clone().err().unwrap());
                    return Err(WavefrontObjError::ParseError(obj.file, row_num, err));
                }
                if coords.len() != 3 {
                    return Err(WavefrontObjError::IncompleteNormal(obj.file, row_num));
                }
                obj.vertex_normals.push(Vec3::new(
                    coords[0].clone().unwrap(),
                    coords[1].clone().unwrap(),
                    coords[2].clone().unwrap(),
                ));
            } else if row.starts_with("vp ") {
                let mut vertex = OptParams::default();
                let coords: Vec<_> = row.split(" ").skip(1).map(|x| x.parse::<f64>()).collect();
                if let Some(c) = coords.iter().find(|c| c.is_err()) {
                    let err = Box::new(c.clone().err().unwrap());
                    return Err(WavefrontObjError::ParseError(obj.file, row_num, err));
                }
                if coords.is_empty() {
                    return Err(WavefrontObjError::IncompleteVertex(obj.file, row_num));
                }
                vertex.u = coords[0].clone().unwrap();
                if coords.len() > 1 {
                    vertex.v = Some(coords[1].clone().unwrap());
                }
                if coords.len() > 2 {
                    vertex.w = Some(coords[2].clone().unwrap());
                }
                obj.parameter_vertices.push(vertex);
            } else if row.starts_with("vt ") {
                let mut vertex = Vec3::ZERO;
                let coords: Vec<_> = row.split(" ").skip(1).map(|x| x.parse::<f64>()).collect();
                if let Some(c) = coords.iter().find(|&c| c.is_err()) {
                    let err = Box::new(c.clone().err().unwrap());
                    return Err(WavefrontObjError::ParseError(obj.file, row_num, err));
                }
                if coords.is_empty() {
                    return Err(WavefrontObjError::IncompleteVertex(obj.file, row_num));
                }
                vertex.x = coords[0].clone().unwrap();
                if coords.len() > 1 {
                    vertex.y = coords[1].clone().unwrap();
                }
                if coords.len() > 2 {
                    vertex.z = coords[2].clone().unwrap();
                }
                obj.texture_coords.push(vertex);
            } else if row.starts_with("l ") {
                let idxs: Vec<_> = row.split(" ").skip(1).map(|x| x.parse::<isize>()).collect();
                if let Some(idx) = idxs.iter().find(|c| c.is_err()) {
                    let err = Box::new(idx.clone().err().unwrap());
                    return Err(WavefrontObjError::ParseError(obj.file, row_num, err));
                }
                obj.lines
                    .push((row_num, idxs.iter().map(|j| j.clone().unwrap()).collect()));
            } else {
                return Err(WavefrontObjError::UnsupportedType(
                    obj.file,
                    row.to_owned(),
                    row_num,
                ));
            }
        }
        obj.validate()
    }

    fn validate(mut self) -> Result<Self, WavefrontObjError> {
        let n_vertices = self.vertices.len();
        let n_texture_coords = self.texture_coords.len();
        let n_normals = self.vertex_normals.len();
        for (line, face) in self.faces.iter_mut() {
            for idx in face.iter_mut() {
                let v_orig = idx.vertex;
                if idx.vertex < 0 {
                    idx.vertex = n_vertices as isize - idx.vertex + 1;
                }
                idx.vertex -= 1;
                if idx.vertex >= n_vertices as isize || idx.vertex < 0 {
                    return Err(WavefrontObjError::InconsistentObject(
                        self.file,
                        *line,
                        GeometryType::Face,
                        GeometryType::Vertex,
                        v_orig,
                    ));
                }
                if let Some(ref mut k) = idx.texture {
                    let v_orig = *k;
                    if *k < 0 {
                        *k = n_texture_coords as isize - *k + 1;
                    }
                    *k -= 1;
                    if *k >= n_texture_coords as isize || *k < 0 {
                        return Err(WavefrontObjError::InconsistentObject(
                            self.file,
                            *line,
                            GeometryType::Face,
                            GeometryType::TextureCoord,
                            v_orig,
                        ));
                    }
                }
                if let Some(ref mut k) = idx.normal {
                    let v_orig = *k;
                    if *k < 0 {
                        *k = n_normals as isize - *k + 1;
                    }
                    *k -= 1;
                    if *k >= n_normals as isize || *k < 0 {
                        return Err(WavefrontObjError::InconsistentObject(
                            self.file,
                            *line,
                            GeometryType::Face,
                            GeometryType::Normal,
                            v_orig,
                        ));
                    }
                }
            }
        }
        for (line, line_element) in self.lines.iter_mut() {
            for vertex in line_element.iter_mut() {
                let v_orig = *vertex;
                if *vertex < 0 {
                    *vertex = n_vertices as isize - *vertex + 1;
                }
                *vertex -= 1;
                if *vertex >= n_vertices as isize || *vertex < 0 {
                    return Err(WavefrontObjError::InconsistentObject(
                        self.file,
                        *line,
                        GeometryType::Line,
                        GeometryType::Vertex,
                        v_orig,
                    ));
                }
            }
        }
        Ok(self)
    }

    pub fn triangulate(&self, material: Material) -> Surface {
        let mut surface = Surface(vec![]);
        for (_, face) in self.faces.iter() {
            let idx0 = face[0].clone();
            let p0 = self.vertices[idx0.vertex as usize].pr3();
            for window in face[1..].windows(2) {
                let idx1 = window[0].clone();
                let idx2 = window[1].clone();
                let p1 = self.vertices[idx1.vertex as usize].pr3();
                let p2 = self.vertices[idx2.vertex as usize].pr3();
                let triangle = Triangle::new(p0, p1 - p0, p2 - p0, material.clone());
                surface.0.push(triangle);
            }
        }
        surface
    }
}

#[derive(Default, Debug, Clone)]
struct ObjIndex {
    vertex: isize,
    texture: Option<isize>,
    normal: Option<isize>,
}

pub enum WavefrontObjError {
    IOError(std::io::Error),
    UnsupportedType(String, String, usize),
    ParseError(String, usize, Box<dyn Error>),
    IncompleteFace(String, usize),
    IncompleteVertex(String, usize),
    IncompleteNormal(String, usize),
    InconsistentObject(String, usize, GeometryType, GeometryType, isize),
    MalformedIndex,
}

impl std::fmt::Display for WavefrontObjError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::IOError(ref err) => write!(f, "{err}"),
            Self::UnsupportedType(ref file, ref typ, line) => {
                write!(
                    f,
                    "{file}: Encountered unsupported type {typ} in line {line}"
                )
            }
            Self::IncompleteFace(ref file, line) => {
                write!(
                    f,
                    "{file}: Not enough vertices to form a face in line {line}"
                )
            }
            Self::IncompleteVertex(ref file, line) => {
                write!(f, "{file}: Not enough data to form a vertex in line {line}")
            }
            Self::IncompleteNormal(ref file, line) => {
                write!(
                    f,
                    "{file}: Not enough data to form a vertex normal in line {line}"
                )
            }

            Self::ParseError(ref file, line, ref err) => {
                write!(f, "{file}: Error parsing line {line}: {err}")
            }
            Self::InconsistentObject(ref file, line, geometry, missing, num) => {
                write!(
                    f,
                    "{file} line {line}: {geometry} references {missing} number {num}, but it doesn't exist"
                )
            }
            Self::MalformedIndex => write!(f, "Malformed index"),
        }
    }
}

impl std::fmt::Debug for WavefrontObjError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
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

#[derive(Debug, Clone, Copy, Default)]
struct OptParams {
    u: f64,
    v: Option<f64>,
    w: Option<f64>,
}

#[derive(Debug, Clone, Copy)]
pub enum GeometryType {
    Line,
    Face,
    Vertex,
    TextureCoord,
    Normal,
}

impl std::fmt::Display for GeometryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Line => write!(f, "Line"),
            Self::Face => write!(f, "Face"),
            Self::Vertex => write!(f, "vertex"),
            Self::TextureCoord => write!(f, "texture coordinate"),
            Self::Normal => write!(f, "vertex normal"),
        }
    }
}
