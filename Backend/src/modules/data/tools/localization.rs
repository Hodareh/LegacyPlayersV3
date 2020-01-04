use crate::modules::data::Data;

pub trait RetrieveLocalization {
  fn get_localization(&self, language_id: u8, localization_id: u32) -> Option<String>;
}

impl RetrieveLocalization for Data {
  fn get_localization(&self, language_id: u8, localization_id: u32) -> Option<String> {
    if language_id == 0 {
      return None;
    }

    self.localization.get(language_id as usize - 1)
        .and_then(|map| map.get(&localization_id)
            .and_then(|localization| Some(localization.clone())))
  }
}