mod consts;

    pub fn get_salally(achievements:f32)->i32
    {
        return (achievements*consts::ZIKYUU).floor();
    }

    pub fn get_tax(source:i32)->i32
    {
        return (source*consts::TAX_RATE).ceil();
    }

    pub fn get_invoice(source:i32)->i32
    {
        return (source*consts::TAX_RATE).ceil();
    }
