use grafton_ndi;

pub enum NdiEvent {
    SourcesFound{
        sources : Vec<grafton_ndi::Source>
    },
}
