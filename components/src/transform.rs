use cgmath::{Euler, Matrix4, Point2, Rad, Vector3};
use cgmath::prelude::Zero;
use specs::{Component, VecStorage};
use utils::{Coord, GfxCoord};

#[derive(Debug)]
pub struct Transform {
    translation: Vector3<GfxCoord>,
    rotation: Euler<Rad<GfxCoord>>,
    scale: Vector3<GfxCoord>,
    dirty: bool,
}

impl Transform {
    pub fn new(pos: Vector3<GfxCoord>, rotation: Euler<Rad<GfxCoord>>, scale: Vector3<GfxCoord>) -> Transform {
        Transform {
            translation: pos,
            rotation: rotation,
            scale: scale,
            dirty: true,
        }
    }

    pub fn new_identity() -> Transform {
        Transform::new(Vector3::zero(), Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)), Vector3::new(1.0, 1.0, 1.0))
    }

    pub fn get_mut_pos(&mut self) -> &mut Vector3<GfxCoord> {
        self.set_dirty();
        &mut self.translation
    }

    pub fn get_mut_rot(&mut self) -> &mut Euler<Rad<GfxCoord>> {
        self.set_dirty();
        &mut self.rotation
    }

    pub fn get_mut_scale(&mut self) -> &mut Vector3<GfxCoord> {
        self.set_dirty();
        &mut self.scale
    }

    pub fn get_pos(&self) -> &Vector3<GfxCoord> {
        &self.translation
    }

    pub fn get_rot(&self) -> &Euler<Rad<GfxCoord>> {
        &self.rotation
    }

    pub fn get_scale(&self) -> &Vector3<GfxCoord> {
        &self.scale
    }

    pub fn get_model(&self) -> Matrix4<GfxCoord> {
        Matrix4::from_translation(self.translation) * Matrix4::from(self.rotation) * Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z)
    }

    pub fn get_gui_offset(&self) -> Point2<Coord> {
        let translation = self.get_pos();
        Point2::new(-translation.x as Coord, -translation.y as Coord)
    }

    fn set_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn take_dirty(&mut self) -> bool {
        let temp = self.dirty;
        self.dirty = false;
        temp
    }
}

impl Component for Transform {
    type Storage = VecStorage<Transform>;
}
