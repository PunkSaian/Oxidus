use macros::vmt_hook;

use crate::{
    sdk::{
        interface::{client_entity_list::ClientEntityList, interface_names},
        module_names,
    },
    util::create_interface,
};

#[vmt_hook]
pub unsafe extern "C" fn add_entity(
    _: &mut ClientEntityList,
    handle_entity: *const (),
    handle: i64,
) {
    dbg!(handle_entity);
    dbg!(handle);
}

pub fn init_esp() {
    let entity_list = create_interface::<ClientEntityList>(
        module_names::CLIENT,
        interface_names::CLIENT_ENTITY_LIST,
    )
    .unwrap();
    for entry in &entity_list.cache {
        if entry.networkable.is_null() {
            continue;
        }
        dbg!(entry);
        unsafe {
            let networkable = &*entry.networkable;
            let client_unknown = &*networkable.get_client_unknown();
            let class = &*networkable.get_client_class();

            let base_entity = &*client_unknown.get_base_entity();
            dbg!(base_entity.m_vecOrigin);
            dbg!(class.class_id);
        }
    }
}
