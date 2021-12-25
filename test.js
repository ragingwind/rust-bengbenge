import { BengBenge } from './index.cjs'

function assert_eq(a, b) {
  if (a !== b) {
    throw new Error(`${a} !== ${b}`)
  }
}

{
  const bbe = new BengBenge();

  bbe.append('192.168.0.1');
  bbe.append('192.168.0.2');
  bbe.append('192.168.0.3');
  bbe.append('192.168.0.4');

  assert_eq('192.168.0.1', bbe.next());
  assert_eq('192.168.0.2', bbe.next());
  assert_eq('192.168.0.3', bbe.next());
  assert_eq('192.168.0.4', bbe.next());
  assert_eq('192.168.0.1', bbe.next());
  assert_eq('192.168.0.2', bbe.next());
  assert_eq('192.168.0.3', bbe.next());
  assert_eq('192.168.0.4', bbe.next());
}

{
  const bbe = new BengBenge();

  bbe.append('192.168.0.1');
  bbe.append('192.168.0.2');
  bbe.append('192.168.0.3');
  bbe.append('192.168.0.4');

  const addreses = [
    '192.168.0.4',
    '192.168.0.3',
    '192.168.0.2',
    '192.168.0.1',
  ];

  let address
  while ((address = addreses.pop())) {
    assert_eq(bbe.next(), address);
  }
}