use cgmath::{ElementWise, Point3, Quaternion};
use graphics_core::model::Model;
use wgpu::wgc::id;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use wgpu::naga::FastHashMap;

#[cfg(feature = "python")]
use pyo3::pyfunction;

#[derive(Default, Debug, Clone)]
pub enum ModelState {
    #[default]
    NoModel,
    Visible(&'static str),
    Invisible(&'static str),
}

impl ModelState {
    //tmp
    pub(crate) fn short_dbg(&self) -> &'static str {
        match self {
            ModelState::NoModel => "NoModel",
            ModelState::Visible(..) => "Visible",
            ModelState::Invisible(..) => "Invisible",
        }
    }
    pub(crate) fn get_model_id(&self) -> Option<&'static str> {
        match self {
            ModelState::NoModel => None,
            ModelState::Visible(id) => Some(*id),
            ModelState::Invisible(id) => Some(*id),
        }
    }
    pub(crate) fn and_then(&self, f: impl Fn(&'static str)) {
        match *self {
            ModelState::Visible(id) => f(id),
            ModelState::Invisible(id) => f(id),
            ModelState::NoModel => {}
        }
    }
}

impl ModelState {
    //TODO!: optimize maybe (later)
    pub fn set_visibility(&mut self, visible: bool) {
        match self {
            ModelState::Invisible(id) if visible => *self = ModelState::Visible(*id),
            ModelState::Visible(id) if !visible => *self = ModelState::Invisible(*id),
            _ => {}
        };
    }

    pub fn set_model<'a>(
        &mut self,
        models: &FastHashMap<&'static str, Model>,
        id: &'static str,
    ) -> Result<(), ()> {
        if !models.contains_key(id) {
            return Err(());
        };
        *self = match self {
            //TODO: visibilty default!
            ModelState::NoModel | ModelState::Visible(_) => ModelState::Visible(id),
            ModelState::Invisible(_) => ModelState::Invisible(id),
        };
        Ok(())
    }

    //maybe unused
    #[inline(always)]
    pub fn no_model() -> Self {
        ModelState::NoModel
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "python", pyclass)]
pub struct Entity { // TODO
    pub id: u32,
    pub model: ModelState,
    pub pos: Point3<f32>,
    pub rotation: Quaternion<f32>,
}

#[cfg_attr(feature = "python", pymethods)]
impl Entity {
    pub fn default(id: u32) -> Self {
        Self {
            id,
            model: Default::default(),
            pos: Point3::new(0., 0., 0.),
            rotation: Quaternion::new(0., 0., 0., 0.),
        }
    }

    #[inline]
    fn set_pos(&mut self, pos: Point3<f32>) {
        self.pos = pos;
    }
    fn set_pos_relative(&mut self, pos: Point3<f32>) {
        self.pos = self.pos.add_element_wise(pos);
    }
    fn get_model_id(&self) -> Option<&'static str> {
        self.model.get_model_id()
    }
    fn set_model(&mut self, model: ModelState) {
        self.model = model;
    }
}

#[cfg_attr(feature = "python", pyfunction)]
#[cdg_attr(feature = "python", pyo3(name = "create_entity"))]
pub fn create_entity_py(id: u32) -> Entity {
    Entity::default(id)
}

//internal state machine for all objects that render
#[derive(Debug)]
pub(crate) struct World {
    id_counter: u32,
    pub entities: FastHashMap<u32, Entity>, //?
    //always valid entity id's
    pub to_be_rendered: HashSet<u32>,
}

impl World {
    pub(crate) fn new() -> World {
        World {
            id_counter: 0,
            entities: FastHashMap::default(),
            to_be_rendered: HashSet::new(),
        }
    }
}

impl World {
    //TODO!: check for possible removal of the return type
    pub(crate) fn insert_entity(&mut self, id: u32, entity: Entity) -> Option<Entity> {
        self.to_be_rendered.insert(id);
        self.entities.insert(id, entity)
    }
    pub(crate) fn contains_entity(&self, id: u32) -> bool {
        self.entities.contains_key(&id)
    }
    pub(crate) fn get_entity(&self, id: u32) -> Option<Entity> {
        self.entities.get(&id).cloned()
    }
    pub(crate) fn delete_entity(&mut self, id: u32) {
        self.to_be_rendered.remove(&id);
        self.entities.remove(&id);
    }
    pub(crate) fn model_id(&mut self, id: u32) -> Option<&str> {
        self.entities
            .get(&id)
            .and_then(|entity| entity.get_model_id())
    }
    pub(crate) fn move_entity(&mut self, id: u32, pos: Point3<f32>) {
        match self.entities.get_mut(&id) {
            Some(entity) => entity.set_pos_relative(pos),
            None => {}
        }
    }
    pub(crate) fn move_entity_rel(&mut self, id: u32, pos: Point3<f32>) {
        //no inspect_mut existing...
        match self.entities.get_mut(&id) {
            Some(entity) => entity.set_pos_relative(entity.pos),
            None => {}
        }
    }
    pub(crate) fn set_model(
        &mut self,
        id: u32,
        models: &FastHashMap<&'static str, Model>,
        model_id: &'static str,
    ) {
        match self.entities.get_mut(&id) {
            Some(entity) => _ = entity.model.set_model(models, model_id),
            None => {}
        }
    }
    //pub(crate) fn get_all(&mut self) -> Entities
}

pub mod commands {
    use crate::commands::Command;
    use crate::game::Game;
    use crate::world::Entity;

    #[cfg(feature = "python")]
    use pyo3::prelude::*;

    #[cfg_attr(feature = "python", pyclass)]
    pub struct CreateEntity(pub u32, pub Entity); // TODO: check need for accessibility of fields from Python

    impl Command for CreateEntity {
        fn execute(&self, game: &mut Game) {
            if !game.world.contains_entity(self.0) {
                game.world.insert_entity(self.0, self.1.clone());
            }
        }
    }

    #[cfg_attr(feature = "python", pyclass)]
    pub struct UpdateEntity(pub u32, pub Entity); // TODO: check need for accessibility of fields from Python
    impl Command for UpdateEntity {
        fn execute(&self, game: &mut Game) {
            if game.world.contains_entity(self.0) {
                game.world.insert_entity(self.0, self.1.clone());
            }
        }
    }
    #[cfg_attr(feature = "python", pyclass)]
    pub struct DeleteEntity(pub u32); // TODO: check need for accessibility of fields from Python
    impl Command for DeleteEntity {
        fn execute(&self, game: &mut Game) {
            game.world.delete_entity(self.0);
        }
    }

    #[cfg_attr(feature = "python", pyclass)]
    pub struct ModifyEntity(pub u32, pub dyn Fn(&mut Entity)); // TODO: check need for accessibility of fields from Python
    impl Command for ModifyEntity {
        fn execute(&self, game: &mut Game) {
            match game.world.get_entity(self.0) {
                None => return,
                Some(ref mut entity) => self.1(entity),
            };
        }
    }
    //we'll see if this is necessary
    #[cfg_attr(feature = "python", pyclass)]
    pub struct SetEntity(pub u32, pub Entity); // TODO: check need for accessibility of fields from Python

    impl Command for SetEntity {
        fn execute(&self, game: &mut Game) {
            game.world.insert_entity(self.0, self.1.clone());
        }
    }
    //the second one is the index in the loaded models
    #[cfg_attr(feature = "python", pyclass)]
    pub struct SetEntityModel(pub u32, pub &'static str); // TODO: check need for accessibility of fields from Python
    impl Command for SetEntityModel {
        fn execute(&self, game: &mut Game) {
            /*println!(
                "after: {:?} ({})",
                game.world
                    .entities
                    .get(&self.0)
                    .map(|entity| { &entity.model }),
                self.0
            );*/
            if let Some(entity) = game.world.entities.get_mut(&self.0) {
                let _ = entity.model.set_model(
                    &game
                        .graphics_state
                        .as_ref()
                        .expect("Always Some(_) during execution")
                        .models,
                    &self.1,
                );
            }
            /*println!(
                "after: {:?} ({})",
                game.world
                    .entities
                    .get(&self.0)
                    .map(|entity| { &entity.model }),
                self.0
            );*/
        }
    }
}
