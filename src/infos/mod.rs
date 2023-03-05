mod infos;
pub mod season;
pub mod npc;
pub mod event;

pub trait Infos {
    fn print(&self) -> ();
    fn get_name(&self) -> &str;
    fn get_season(&self) -> &str;
    fn get_days(&self) -> u8;
    fn get_day(&self) -> &str;
}