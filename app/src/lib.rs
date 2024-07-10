#![no_std]

use sails_rtl::{
    prelude::*,
    gstd::{
        gprogram,
        groute
    }
};

pub mod states;
pub mod services;


use crate::states::state::{
    STATE,
    CustomStruct
};


use services::service::Service;


#[derive(Default)]
pub struct Program;

#[gprogram]
impl Program {
   
    pub fn new() -> Self {
        unsafe {
            STATE = Some(
                CustomStruct::default()
            );
        };

        Self
    }

    
    #[groute("Template")]
    pub fn template_svc(&self) -> Service {
        Service::new()
    }
}