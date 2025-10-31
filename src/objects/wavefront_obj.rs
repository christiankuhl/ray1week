use std::{collections::HashMap, error::Error, fs::read_to_string, path::Path, sync::Arc};

use image::{ImageError, ImageReader, Rgb32FImage};

use crate::{
    colour::Colour,
    linalg::{Point3, Vec3, Vec4},
    material::{Lambertian, Material, Metal},
    objects::{IntoPrimitives, Object, Triangle},
    texture::{ImageTexture, UVTriangle},
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
    mtllib: Option<MtlLib>,
    mat_assign: HashMap<usize, Arc<String>>,
}

impl WavefrontObj {
    #[allow(clippy::field_reassign_with_default)]
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, WavefrontObjError> {
        let data = read_to_string(&path)?;
        let mut obj = Self::default();
        obj.file = path_ref_to_string(&path);
        let mut current_material: Option<Arc<String>> = None;
        let mut current_face = 0;
        for (row_num, row) in data.split("\n").enumerate() {
            let row_num = row_num + 1;
            if row.starts_with("#")
                || row.is_empty()
                || row.starts_with("o ")
                || row.starts_with("s ")
            {
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
                    if let Some(ref material) = current_material {
                        obj.mat_assign.insert(current_face, Arc::clone(material));
                    }
                    current_face += 1;
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
            } else if row.starts_with("mtllib ") {
                if let Some(file) = row.split(" ").nth(1) {
                    let path = path.as_ref().with_file_name(file);
                    obj.mtllib = Some(MtlLib::from_file(path)?);
                } else {
                    let err = Box::new(WavefrontObjError::MissingArgument("mtllib".to_owned()));
                    return Err(WavefrontObjError::ParseError(obj.file, row_num, err));
                }
            } else if row.starts_with("usemtl ") {
                if let Some((_, mat)) = row.split_once(" ") {
                    current_material = Some(Arc::new(mat.to_owned()));
                } else {
                    let err = Box::new(WavefrontObjError::MissingArgument("usemtl".to_owned()));
                    return Err(WavefrontObjError::ParseError(obj.file, row_num, err));
                }
            } else {
                return Err(WavefrontObjError::UnsupportedType(
                    obj.file,
                    row.split(" ").next().unwrap().to_owned(),
                    row_num,
                ));
            }
        }
        obj.validate()
    }

    fn validate(mut self) -> Result<Self, WavefrontObjError> {
        if !self.mat_assign.is_empty() && self.mtllib.is_none() {
            return Err(WavefrontObjError::NoMaterial(self.file));
        }
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

    fn _triangulate(
        &self,
        cb: impl Fn(Point3, Vec3, Vec3, usize, &ObjIndex, &ObjIndex, &ObjIndex) -> Object,
    ) -> Surface {
        let mut surface = Surface(vec![]);
        for (face_idx, (_, face)) in self.faces.iter().enumerate() {
            let idx0 = face[0].clone();
            let p0 = self.vertices[idx0.vertex as usize].pr3();
            for window in face[1..].windows(2) {
                let idx1 = window[0].clone();
                let idx2 = window[1].clone();
                let p1 = self.vertices[idx1.vertex as usize].pr3();
                let p2 = self.vertices[idx2.vertex as usize].pr3();
                let triangle = cb(p0, p1 - p0, p2 - p0, face_idx, &idx0, &idx1, &idx2);
                surface.0.push(triangle);
            }
        }
        surface
    }

    pub fn triangulate_with_material(&self, material: Material) -> Surface {
        self._triangulate(|q, u, v, _, _, _, _| Triangle::new(q, u, v, material.clone()))
    }

    pub fn triangulate(&self) -> Result<Surface, WavefrontObjError> {
        match self.mtllib {
            None => Err(WavefrontObjError::NoMaterial(self.file.clone())),
            Some(ref mtllib) => {
                let materials = mtllib.build()?;
                let default = Metal::new(Colour::new(0.15, 0.15, 0.73), 0.1);
                let surface = self._triangulate(|q, u, v, face_idx, idx0, idx1, idx2| {
                    let uv0 = idx0.texture.map(|idx| self.texture_coords[idx as usize]);
                    let uv1 = idx1.texture.map(|idx| self.texture_coords[idx as usize]);
                    let uv2 = idx2.texture.map(|idx| self.texture_coords[idx as usize]);
                    let mat = if let Some(mat_name) = self.mat_assign.get(&face_idx) {
                        let mat_idx = mtllib.material_names.get(mat_name.as_ref()).unwrap();
                        materials.get(mat_idx).unwrap_or(&default)
                    } else {
                        &default
                    };
                    let triangle = Triangle::new(q, u, v, mat.clone());
                    if uv0.is_none() || uv1.is_none() || uv2.is_none() {
                        triangle
                    } else {
                        let p0 = uv0.unwrap();
                        let p1 = uv1.unwrap() - p0;
                        let p2 = uv2.unwrap() - p0;
                        UVTriangle::new(p0, p1, p2, triangle)
                    }
                });
                Ok(surface)
            }
        }
    }
}

#[derive(Debug)]
pub struct MtlLib {
    material_names: HashMap<String, usize>,
    materials: HashMap<usize, Mtl>,
}

impl MtlLib {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, WavefrontObjError> {
        let file = path_ref_to_string(&path);
        let mut lib = Self {
            material_names: HashMap::new(),
            materials: HashMap::new(),
        };
        let data = read_to_string(&path)?;
        let mut mtls = 0;
        let mut mtl = Mtl::default();
        let mut mat_name: String = String::new();
        for (row_num, row) in data.split("\n").enumerate() {
            let row_num = row_num + 1;
            let row = row.trim();
            if row.starts_with("# ") || row.is_empty() {
                continue;
            } else if row.starts_with("newmtl ") {
                if let Some(name) = row.split(" ").nth(1) {
                    mat_name = name.to_owned();
                } else {
                    let err = Box::new(WavefrontObjError::MissingArgument("newmtl".to_owned()));
                    return Err(WavefrontObjError::ParseError(file, row_num, err));
                }
                if mtls > 0 {
                    lib.material_names.insert(mat_name.clone(), mtls);
                    lib.materials.insert(mtls, mtl);
                    mtl = Mtl::default();
                }
                mtls += 1;
            } else if row.starts_with("Ka ") {
                mtl.ambient = parse_colour(row, row_num, &file)?;
            } else if row.starts_with("Kd ") {
                mtl.diffuse = parse_colour(row, row_num, &file)?;
            } else if row.starts_with("Ks ") {
                mtl.specular = parse_colour(row, row_num, &file)?;
            } else if row.starts_with("Ke ") {
                mtl.emissive = parse_colour(row, row_num, &file)?;
            } else if row.starts_with("Tf ") {
                mtl.transmission_filter = parse_colour(row, row_num, &file)?;
            } else if row.starts_with("Ns ") {
                mtl.exponent = parse_single_float(row, row_num, &file)?;
            } else if row.starts_with("Ni ") {
                mtl.density = parse_single_float(row, row_num, &file)?;
            } else if row.starts_with("d ") || row.starts_with("Tr ") {
                mtl.transmission = parse_single_float(row, row_num, &file)?;
                if row.starts_with("d ") {
                    mtl.transmission = 1.0 - mtl.transmission;
                }
            } else if row.starts_with("illum ") {
                mtl.illumination = IlluminationModel::parse(row, row_num, &file)?;
            } else if row.starts_with("map_Kd ") {
                if let Some(file) = row.split(" ").nth(1) {
                    let path = path.as_ref().with_file_name(file);
                    mtl.diffuse_texture = Some(ImageReader::open(path)?.decode()?.into_rgb32f());
                } else {
                    let err = Box::new(WavefrontObjError::MissingArgument("map_Kd".to_owned()));
                    return Err(WavefrontObjError::ParseError(file, row_num, err));
                }
            } else {
                return Err(WavefrontObjError::UnsupportedType(
                    file,
                    row.split(" ").next().unwrap().to_owned(),
                    row_num,
                ));
            }
        }
        if mtls > 0 {
            lib.material_names.insert(mat_name.clone(), mtls);
            lib.materials.insert(mtls, mtl);
        }
        Ok(lib)
    }

    fn build(&self) -> Result<HashMap<usize, Material>, WavefrontObjError> {
        let mut result = HashMap::new();
        for (idx, mtl) in self.materials.iter() {
            // TODO: Implement some actual logic here
            match mtl.diffuse_texture {
                None => result.insert(*idx, Lambertian::new(mtl.diffuse)),
                Some(ref buffer) => result.insert(
                    *idx,
                    Lambertian::from_texture(ImageTexture::from_buffer(buffer)),
                ),
            };
        }
        Ok(result)
    }
}

#[derive(Default, Debug)]
struct Mtl {
    ambient: Colour,
    diffuse: Colour,
    specular: Colour,
    exponent: f64,
    transmission: f64,
    transmission_filter: Colour,
    emissive: Colour,
    density: f64,
    illumination: IlluminationModel,
    diffuse_texture: Option<Rgb32FImage>,
}

#[derive(Default, Debug, Clone)]
struct ObjIndex {
    vertex: isize,
    texture: Option<isize>,
    normal: Option<isize>,
}

pub enum WavefrontObjError {
    IOError(std::io::Error),
    ImageError(ImageError),
    UnsupportedType(String, String, usize),
    ParseError(String, usize, Box<dyn Error>),
    IncompleteFace(String, usize),
    IncompleteVertex(String, usize),
    IncompleteNormal(String, usize),
    InconsistentObject(String, usize, GeometryType, GeometryType, isize),
    MalformedIndex,
    MissingArgument(String),
    IncompleteColour(String, usize),
    UnknownIlluminationModel(String, usize, String),
    NoMaterial(String),
}

impl std::fmt::Display for WavefrontObjError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::IOError(ref err) => write!(f, "{err}"),
            Self::ImageError(ref err) => write!(f, "{err}"),
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
            Self::MissingArgument(ref directive) => {
                write!(f, "Missing argument for directive {directive}")
            }
            Self::IncompleteColour(ref file, line) => {
                write!(f, "{file}: Not enough data to form a colour in line {line}")
            }
            Self::UnknownIlluminationModel(ref file, line, ref mode) => {
                write!(
                    f,
                    "{file}, line {line}: Encountered unknown illumination model '{mode}'"
                )
            }
            Self::NoMaterial(ref file) => write!(f, "File {file} contains no mtllib directive"),
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

impl From<ImageError> for WavefrontObjError {
    fn from(value: ImageError) -> Self {
        Self::ImageError(value)
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

#[derive(Debug)]
enum IlluminationModel {
    // 0. Color on and Ambient off
    ColourNoAmbient,
    // 1. Color on and Ambient on
    ColourAmbient,
    // 2. Highlight on
    Highlight,
    // 3. Reflection on and Ray trace on
    ReflectionRaytrace,
    // 4. Transparency: Glass on, Reflection: Ray trace on
    GlassReflection,
    // 5. Reflection: Fresnel on and Ray trace on
    ReflectionFresnel,
    // 6. Transparency: Refraction on, Reflection: Fresnel off and Ray trace on
    RefractionNoFresnel,
    // 7. Transparency: Refraction on, Reflection: Fresnel on and Ray trace on
    RefractionFresnel,
    // 8. Reflection on and Ray trace off
    ReflectionOnly,
    // 9. Transparency: Glass on, Reflection: Ray trace off
    GlassNoReflection,
    // 10. Casts shadows onto invisible surfaces
    Shadows,
}

impl IlluminationModel {
    fn parse(row: &str, row_num: usize, file: &str) -> Result<Self, WavefrontObjError> {
        if let Some((_directive, value)) = row.split_once(" ") {
            match value {
                "0" => Ok(Self::ColourNoAmbient),
                "1" => Ok(Self::ColourAmbient),
                "2" => Ok(Self::Highlight),
                "3" => Ok(Self::ReflectionRaytrace),
                "4" => Ok(Self::GlassReflection),
                "5" => Ok(Self::ReflectionFresnel),
                "6" => Ok(Self::RefractionNoFresnel),
                "7" => Ok(Self::RefractionFresnel),
                "8" => Ok(Self::ReflectionOnly),
                "9" => Ok(Self::GlassNoReflection),
                "10" => Ok(Self::Shadows),
                _ => Err(WavefrontObjError::UnknownIlluminationModel(
                    file.to_owned(),
                    row_num,
                    value.to_owned(),
                )),
            }
        } else {
            let err = Box::new(WavefrontObjError::MissingArgument(row.to_owned()));
            Err(WavefrontObjError::ParseError(file.to_owned(), row_num, err))
        }
    }
}

impl Default for IlluminationModel {
    fn default() -> Self {
        Self::ColourNoAmbient
    }
}

fn path_ref_to_string(path: impl AsRef<Path>) -> String {
    path.as_ref()
        .as_os_str()
        .to_owned()
        .into_string()
        .expect("You have done something so esoteric, you probably deserve this.")
}

fn parse_colour(row: &str, row_num: usize, file: &str) -> Result<Colour, WavefrontObjError> {
    let components: Vec<_> = row.split(" ").skip(1).map(|s| s.parse::<f64>()).collect();
    if components.len() != 3 {
        return Err(WavefrontObjError::IncompleteColour(
            file.to_owned(),
            row_num,
        ));
    }
    if let Some(c) = components.iter().find(|s| s.is_err()) {
        let err = Box::new(c.clone().err().unwrap());
        return Err(WavefrontObjError::ParseError(file.to_owned(), row_num, err));
    }
    Ok(Colour::new(
        components[0].clone().unwrap(),
        components[1].clone().unwrap(),
        components[2].clone().unwrap(),
    ))
}

fn parse_single_float(row: &str, row_num: usize, file: &str) -> Result<f64, WavefrontObjError> {
    if let Some((_directive, value)) = row.split_once(" ") {
        value
            .parse()
            .map_err(|err| WavefrontObjError::ParseError(file.to_owned(), row_num, Box::new(err)))
    } else {
        let err = Box::new(WavefrontObjError::MissingArgument(row.to_owned()));
        Err(WavefrontObjError::ParseError(file.to_owned(), row_num, err))
    }
}
