#![allow(incomplete_features)]
#![feature(generic_associated_types)]
#![feature(associated_type_defaults)]

// A Collection trait and collection families. Based on
// http://smallcultfollowing.com/babysteps/blog/2016/11/03/
// associated-type-constructors-part-2-family-traits/

trait Collection<T> {
    type Iter<'iter>: Iterator<Item=&'iter T>;
    type Family: CollectionFamily;
    // Test associated type defaults with parameters
    type Sibling<U>: Collection<U> =
        <<Self as Collection<T>>::Family as CollectionFamily>::Member<U>;
    //~^^ ERROR type-generic associated types are not yet implemented

    fn empty() -> Self;

    fn add(&mut self, value: T);

    fn iterate<'iter>(&'iter self) -> Self::Iter<'iter>;
}

trait CollectionFamily {
    type Member<T>: Collection<T, Family = Self>;
    //~^ ERROR type-generic associated types are not yet implemented
}

struct VecFamily;

impl CollectionFamily for VecFamily {
    type Member<T> = Vec<T>;
}

impl<T> Collection<T> for Vec<T> {
    type Iter<'iter> = std::slice::Iter<'iter, T>;
    type Family = VecFamily;

    fn empty() -> Self {
        Vec::new()
    }

    fn add(&mut self, value: T) {
        self.push(value)
    }

    fn iterate<'iter>(&'iter self) -> Self::Iter<'iter> {
        self.iter()
    }
}

fn floatify<C>(ints: &C) -> <<C as Collection<i32>>::Family as CollectionFamily>::Member<f32>
where
    C: Collection<i32>,
{
    let mut res = C::Family::Member::<f32>::empty();
    for &v in ints.iterate() {
        res.add(v as f32);
    }
    res
}

fn floatify_sibling<C>(ints: &C) -> <C as Collection<i32>>::Sibling<f32>
where
    C: Collection<i32>,
{
    let mut res = C::Family::Member::<f32>::empty();
    for &v in ints.iterate() {
        res.add(v as f32);
    }
    res
}

fn use_floatify() {
    let a = vec![1i32, 2, 3];
    let b = floatify(a);
    println!("{}", b.iterate().next());
    let c = floatify_sibling(a);
    println!("{}", c.iterate().next());
}

fn main() {}
