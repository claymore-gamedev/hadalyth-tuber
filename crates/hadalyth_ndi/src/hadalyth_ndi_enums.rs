use grafton_ndi;

pub enum NdiEvent {
    SourcesFound(Vec<grafton_ndi::Source>)
}