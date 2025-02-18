#debugruntest-compatible
# Copyright (c) Meta Platforms, Inc. and affiliates.
# Copyright (c) Mercurial Contributors.
#
# This software may be used and distributed according to the terms of the
# GNU General Public License version 2 or any later version.

  $ hg init repo
  $ cd repo
  $ echo a > a
  $ hg ci -Am t
  adding a

  $ hg mv a b
  $ hg ci -Am t1
  $ hg debugrename b
  b renamed from a:b789fdd96dc2f3bd229c1dd8eedf0fc60e2b68e3

  $ hg mv b a
  $ hg ci -Am t2
  $ hg debugrename a
  a renamed from b:37d9b5d994eab34eda9c16b195ace52c7b129980

  $ hg debugrename --rev 1 b
  b renamed from a:b789fdd96dc2f3bd229c1dd8eedf0fc60e2b68e3
