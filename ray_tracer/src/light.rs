use crate::{
    color::{black, Color},
    material::Material,
    tuple::*,
};

pub struct PointLight {
    pub position: Tuple,
    pub intensity: Color,
}

pub fn point_light(position: Tuple, intensity: Color) -> PointLight {
    PointLight {
        position,
        intensity,
    }
}

pub fn lighting(
    material: &Material,
    light: &PointLight,
    p: Tuple,
    eyev: Tuple,
    normalv: Tuple,
) -> Color {
    let effective_color = material.color * light.intensity;
    let lightv = (light.position - p).normalize();
    let ambient = effective_color * material.ambient;
    let diffuse: Color;
    let specular: Color;

    let light_dot_normal = lightv.dot(normalv);
    if light_dot_normal < 0. {
        diffuse = black();
        specular = black();
    } else {
        diffuse = effective_color * material.diffuse * light_dot_normal;

        let reflectv = reflect(-lightv, normalv);
        let reflect_dot_eye = reflectv.dot(eyev);

        if reflect_dot_eye <= 0. {
            specular = black();
        } else {
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = light.intensity * material.specular * factor;
        }
    }

    return ambient + diffuse + specular;
}
