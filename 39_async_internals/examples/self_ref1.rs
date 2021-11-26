struct SelfReference<'a> {
    name: String,
    name_ref: Option<&'a str>,
}

fn main() {
    let mut data = SelfReference {
        name: "Tyr".into(),
        name_ref: None,
    };
    data.name_ref = Some(&data.name);

    // 如果 move，直接编译不过（std::mem::swap 也是如此）
    // move_it(data);
}

#[allow(dead_code)]
fn move_it(data: SelfReference) -> SelfReference {
    data
}
