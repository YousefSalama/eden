#chg-compatible

#require gzip gunzip tr

Test encode/decode filters

  $ hg init repo
  $ cd repo
  $ readconfig <<EOF
  > [encode]
  > not.gz = tr [:lower:] [:upper:]
  > *.gz = gzip -d
  > [decode]
  > not.gz = tr [:upper:] [:lower:]
  > *.gz = gzip
  > EOF
  $ echo "this is a test" | gzip > a.gz
  $ echo "this is a test" > not.gz
  $ hg add *
  $ hg ci -m "test"

no changes

  $ hg status
  $ touch *

no changes

  $ hg status

check contents in repo are encoded

  $ hg debugdata a.gz 0
  this is a test
  $ hg debugdata not.gz 0
  THIS IS A TEST

check committed content was decoded

  $ gunzip < a.gz
  this is a test
  $ cat not.gz
  this is a test
  $ rm *
  $ hg co -C
  2 files updated, 0 files merged, 0 files removed, 0 files unresolved

check decoding of our new working dir copy

  $ gunzip < a.gz
  this is a test
  $ cat not.gz
  this is a test

check hg cat operation

  $ hg cat a.gz
  this is a test
  $ hg cat --decode a.gz | gunzip
  this is a test
  $ mkdir subdir
  $ cd subdir
  $ hg -R .. cat ../a.gz
  this is a test
  $ hg -R .. cat --decode ../a.gz | gunzip
  this is a test

  $ cd ..
