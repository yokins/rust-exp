struct Dog {
    name: String,
}

impl Dog {
    fn name(&self) -> String {
        self.name() // 调用 Dog 自己的 name 方法
    }
}

trait Animal {
    fn name(&self) -> String;
}

struct Cat {
    name: String,
}

impl Cat {
    fn name(&self) -> String {
        self.name() // 调用 Dog 自己的 name 方法
    }
}

fn get_name<T>(animal: T) -> String
where
    T: Animal,
{
    animal.name()
}

fn main() {
    let dog = Dog {
        name: "dog".to_string(),
    };

    let cat = Cat {
        name: "cat".to_string(),
    };

    let name = get_name(dog);

    println!("hello world {}", name);
}
