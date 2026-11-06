use crate::model::bone::Bone;

pub struct Skeleton {
    label: String,
    bones: Vec<Bone>,
}

impl Skeleton {
    pub const fn new(label: String) -> Self {
        Skeleton {
            label,
            bones: Vec::new(),
        }
    }

    pub const fn from_bones(label: String, bones: Vec<Bone>) -> Self {
        Skeleton { label, bones }
    }

    pub fn add_bone(mut self, bone: Bone) -> Self {
        self.bones.push(bone);
        self
    }

    pub fn rename(mut self, new_label: String) -> Self {
        self.label = new_label;
        self
    }
}

//TODO: Helper -> sortir une liste des enfant en fonction d'un bone parent
