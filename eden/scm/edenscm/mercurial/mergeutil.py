# Portions Copyright (c) Meta Platforms, Inc. and affiliates.
#
# This software may be used and distributed according to the terms of the
# GNU General Public License version 2.

# mergeutil.py - help for merge processing in mercurial
#
# Copyright 2005-2007 Olivia Mackall <olivia@selenic.com>
#
# This software may be used and distributed according to the terms of the
# GNU General Public License version 2 or any later version.

from __future__ import absolute_import

from . import error
from .i18n import _


def checkunresolved(ms):
    if list(ms.unresolved()):
        raise error.Abort(_("unresolved merge conflicts " "(see 'hg help resolve')"))
    if ms.mdstate() != "s" or list(ms.driverresolved()):
        raise error.Abort(
            _("driver-resolved merge conflicts"),
            hint=_('run "hg resolve --all" to resolve'),
        )
