pub trait ValidData: ToString + Default {}

impl ValidData for String {}
