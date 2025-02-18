# Portions Copyright (c) Meta Platforms, Inc. and affiliates.
#
# This software may be used and distributed according to the terms of the
# GNU General Public License version 2.

# Copyright Olivia Mackall <olivia@selenic.com> and others
#
# This software may be used and distributed according to the terms of the
# GNU General Public License version 2 or any later version.

# dirstatenonnormalcheck.py - extension to check the consistency of the
# dirstate's non-normal map
#
# For most operations on dirstate, this extensions checks that the nonnormalset
# contains the right entries.
# It compares the nonnormal file to a nonnormalset built from the map of all
# the files in the dirstate to check that they contain the same files.

from __future__ import absolute_import

from edenscm.mercurial import dirstate, extensions


def nonnormalentries(dmap):
    """Compute nonnormal entries from dirstate's dmap"""
    res = set()
    for f, e in dmap.iteritems():
        if e[0] != "n" or e[3] == -1:
            res.add(f)
    return res


def checkconsistency(ui, orig, dmap, _nonnormalset, label):
    """Compute nonnormalset from dmap, check that it matches _nonnormalset"""
    nonnormalcomputedmap = nonnormalentries(dmap)
    if _nonnormalset != nonnormalcomputedmap:
        ui.develwarn("%s call to %s\n" % (label, orig), config="dirstate")
        ui.develwarn("inconsistency in nonnormalset\n", config="dirstate")
        ui.develwarn("[nonnormalset] %s\n" % _nonnormalset, config="dirstate")
        ui.develwarn("[map] %s\n" % nonnormalcomputedmap, config="dirstate")


def _checkdirstate(orig, self, arg):
    """Check nonnormal set consistency before and after the call to orig"""
    checkconsistency(self._ui, orig, self._map, self._map.nonnormalset, "before")
    r = orig(self, arg)
    checkconsistency(self._ui, orig, self._map, self._map.nonnormalset, "after")
    return r


def extsetup(ui):
    """Wrap functions modifying dirstate to check nonnormalset consistency"""
    dirstatecl = dirstate.dirstate
    devel = ui.configbool("devel", "all-warnings")
    paranoid = ui.configbool("experimental", "nonnormalparanoidcheck")
    if devel:
        extensions.wrapfunction(dirstatecl, "_writedirstate", _checkdirstate)
        if paranoid:
            # We don't do all these checks when paranoid is disable as it would
            # make the extension run very slowly on large repos
            extensions.wrapfunction(dirstatecl, "normallookup", _checkdirstate)
            extensions.wrapfunction(dirstatecl, "otherparent", _checkdirstate)
            extensions.wrapfunction(dirstatecl, "normal", _checkdirstate)
            extensions.wrapfunction(dirstatecl, "write", _checkdirstate)
            extensions.wrapfunction(dirstatecl, "add", _checkdirstate)
            extensions.wrapfunction(dirstatecl, "remove", _checkdirstate)
            extensions.wrapfunction(dirstatecl, "merge", _checkdirstate)
            extensions.wrapfunction(dirstatecl, "untrack", _checkdirstate)
