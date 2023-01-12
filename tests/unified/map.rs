use crate::prelude::*;

#[test]
fn foo() -> Result<(), std::fmt::Error> {
  let mut s = String::new();
  let mut t = HashMapNZ64::<u64>::new();

  for i in 1 ..= 100 {
    let k = NonZeroU64::new(i).unwrap();
    t.insert(k, 10 * i);
  }

  writeln!(s, "load = {:#?}", map::internal::load(&t))?;

  for &key in t.sorted_keys().iter() {
    assert!(t.contains_key(key));
  }

  for i in 1 ..= 100 {
    if i & 1 == 0 {
      let k = NonZeroU64::new(i).unwrap();
      assert!(t.remove(k).is_some());
    }
  }

  writeln!(s, "load = {:#?}", map::internal::load(&t))?;

  for &key in t.sorted_keys().iter() {
    assert!(t.contains_key(key));
  }

  writeln!(s, "{:#?}", t)?;

  expect![[r#"
      load = 0.3787878787878788
      load = 0.1893939393939394
      {
          1: 10,
          3: 30,
          5: 50,
          7: 70,
          9: 90,
          11: 110,
          13: 130,
          15: 150,
          17: 170,
          19: 190,
          21: 210,
          23: 230,
          25: 250,
          27: 270,
          29: 290,
          31: 310,
          33: 330,
          35: 350,
          37: 370,
          39: 390,
          41: 410,
          43: 430,
          45: 450,
          47: 470,
          49: 490,
          51: 510,
          53: 530,
          55: 550,
          57: 570,
          59: 590,
          61: 610,
          63: 630,
          65: 650,
          67: 670,
          69: 690,
          71: 710,
          73: 730,
          75: 750,
          77: 770,
          79: 790,
          81: 810,
          83: 830,
          85: 850,
          87: 870,
          89: 890,
          91: 910,
          93: 930,
          95: 950,
          97: 970,
          99: 990,
      }
  "#]].assert_eq(&s);

  Ok(())
}


#[test]
fn test_keys() -> Result<(), std::fmt::Error> {
  let mut s = String::new();
  let mut t = HashMapNZ64::<u64>::new();

  let keys = [
    10,
    5,
    100,
    13,
    1000,
    17,
    10000
  ].map(|x| NonZeroU64::new(x).unwrap());

  for &key in keys.iter() {
    t.insert(key, u64::from(key) - 1);
  }

  writeln!(s, "{:?}", t.items_sorted_by_key())?;

  expect![[r#"
      [(5, 4), (10, 9), (13, 12), (17, 16), (100, 99), (1000, 999), (10000, 9999)]
  "#]].assert_eq(&s);

  Ok(())
}

#[test]
fn test_basic() -> Result<(), std::fmt::Error> {
  let mut s = String::new();
  let mut t = HashMapNZ64::<u64>::new();

  let key = NonZeroU64::new(13).unwrap();

  writeln!(s, "{:?} <- t.len()", t.len())?;
  writeln!(s, "{:?} <- t.is_empty()", t.is_empty())?;
  writeln!(s, "{:?} <- t.contains_key({:?})", t.contains_key(key), key)?;
  writeln!(s, "{:?} <- t.get({:?})", t.get(key), key)?;
  writeln!(s, "{:?} <- t.get_mut({:?})", t.get_mut(key), key)?;
  writeln!(s, "{:?} <- t.insert({:?}, {:?})", t.insert(key, 42), key, 42)?;
  writeln!(s, "{:?} <- t.len()", t.len())?;
  writeln!(s, "{:?} <- t.is_empty()", t.is_empty())?;
  writeln!(s, "{:?} <- t.contains_key({:?})", t.contains_key(key), key)?;
  writeln!(s, "{:?} <- t.get({:?})", t.get(key), key)?;
  writeln!(s, "{:?} <- t.get_mut({:?})", t.get_mut(key), key)?;
  writeln!(s, "{:?} <- t.remove({:?})", t.remove(key), key)?;
  writeln!(s, "{:?} <- t.len()", t.len())?;
  writeln!(s, "{:?} <- t.is_empty()", t.is_empty())?;
  writeln!(s, "{:?} <- t.contains_key({:?})", t.contains_key(key), key)?;
  writeln!(s, "{:?} <- t.get({:?})", t.get(key), key)?;
  writeln!(s, "{:?} <- t.get_mut({:?})", t.get_mut(key), key)?;

  expect![[r#"
      0 <- t.len()
      true <- t.is_empty()
      false <- t.contains_key(13)
      None <- t.get(13)
      None <- t.get_mut(13)
      None <- t.insert(13, 42)
      1 <- t.len()
      false <- t.is_empty()
      true <- t.contains_key(13)
      Some(42) <- t.get(13)
      Some(42) <- t.get_mut(13)
      Some(42) <- t.remove(13)
      0 <- t.len()
      true <- t.is_empty()
      false <- t.contains_key(13)
      None <- t.get(13)
      None <- t.get_mut(13)
  "#]].assert_eq(&s);

  Ok(())
}
