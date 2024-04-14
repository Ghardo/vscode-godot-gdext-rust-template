use godot::engine::{ 
    Node3D, INode3D
};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct MyNode {
    base: Base<Node3D>,
}

#[godot_api]
impl MyNode {
}

#[godot_api]
impl INode3D for MyNode {
    fn init(base: Base<Node3D>) -> Self {
        MyNode { base }
    }
        
    fn process(&mut self, _delta: f64,) {
        unimplemented !()
    }
    
    fn physics_process(&mut self, _delta: f64,) {
        unimplemented !()
    }
    
    fn ready(&mut self,) {
        unimplemented !()
    }
}