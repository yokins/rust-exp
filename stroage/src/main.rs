use std::{
    fs,
    io::{Read, Write},
};

trait Storage {
    fn restore(&mut self) -> Result<(), std::io::Error>;
    fn save(&mut self, good: Good) -> Result<(), std::io::Error>;
    fn update(&mut self, update_good_params: UpdateGoodParams) -> Result<(), std::io::Error>;
    fn save_update(&mut self) -> Result<(), std::io::Error>;
}

struct UpdateGoodParams {
    name: String,
    key: String,
    value: u32,
}

#[derive(Debug, Clone)]
struct Good {
    name: String,
    price: u32,
    count: u32,
}

#[derive(Debug)]
struct Warehouse {
    goods: Vec<Good>,
}

impl Storage for Warehouse {
    fn restore(&mut self) -> Result<(), std::io::Error> {
        let mut f = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("goods.txt")
            .expect("文件打开失败");
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        let goods: Vec<Good> = content
            .lines()
            .map(|line| line.splitn(3, '\t').collect::<Vec<&str>>())
            .map(|v| (v[0], v[1], v[2]))
            .map(|(name, price, count)| Good {
                name: String::from(name),
                price: price.parse::<u32>().unwrap_or(0),
                count: count.parse::<u32>().unwrap_or(0),
            })
            .collect();
        self.goods = goods;

        println!("当前仓库中有 {} 件商品:", self.goods.len());
        println!("-----------------------------------");
        println!("{:<20} {:<10} {:<10}", "商品名", "单价", "数量");
        println!("-----------------------------------");
        for good in &self.goods {
            println!("{:<20} {:<10} {:<10}", good.name, good.price, good.count);
        }
        println!("-----------------------------------");
        Ok(())
    }

    fn save(&mut self, good: Good) -> Result<(), std::io::Error> {
        let mut file = fs::OpenOptions::new()
            .create(true) // 如果文件不存在则创建
            .append(true) // 以追加模式打开文件
            .open("goods.txt")?;

        // 写入商品信息到文件
        writeln!(file, "{}\t{}\t{}", good.name, good.price, good.count)?;
        self.goods.push(good);
        Ok(())
    }

    fn update(&mut self, update_good_params: UpdateGoodParams) -> Result<(), std::io::Error> {
        if let Some(good) = self
            .goods
            .iter_mut()
            .find(|v| v.name == update_good_params.name)
        {
            match update_good_params.key.as_str() {
                "price" => good.price = update_good_params.value,
                "count" => good.count = update_good_params.value,
                _ => {
                    println!("无效的键: {}", update_good_params.key);
                }
            }
        }

        Ok(())
    }

    fn save_update(&mut self) -> Result<(), std::io::Error> {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("goods.txt")?;

        for good in &self.goods {
            writeln!(file, "{}\t{}\t{}", good.name, good.price, good.count)?;
        }

        Ok(())
    }
}

fn main() {
    let mut storage: Warehouse = Warehouse { goods: vec![] };
    let action = std::env::args().nth(1).expect("没有输入操作指令");
    if action == "list" {
        storage.restore().unwrap();
        print!("现在仓库中有{}件商品/n", storage.goods.len());
    } else if action == "save" {
        let good_name: String = std::env::args().nth(2).expect("没有输入商品名称");
        let good_price = std::env::args()
            .nth(3)
            .expect("没有输入商品价格")
            .parse::<u32>()
            .expect("商品价格必须是数字");
        let good_count = std::env::args()
            .nth(4)
            .expect("没有输入商品数量")
            .parse::<u32>()
            .expect("商品数量必须是数字");

        let good = Good {
            name: good_name,
            price: good_price,
            count: good_count,
        };
        storage.save(good).unwrap();
    } else if action == "update" {
        let good_name: String = std::env::args().nth(2).expect("没有输入商品名称");
        let key: String = std::env::args().nth(3).expect("没有输入商品属性");
        let value = std::env::args()
            .nth(4)
            .expect("没有输入商品属性值")
            .parse::<u32>()
            .expect("商品属性值必须是数字");

        let good: UpdateGoodParams = UpdateGoodParams {
            name: good_name,
            key,
            value,
        };

        storage.restore().unwrap();
        storage.update(good).unwrap();
        storage.save_update().unwrap();
    }
}
