#!/usr/bin/perl

use strict;
use warnings;

use Test::More tests => 1;

use Test::Differences qw( eq_or_diff );

{
    # TEST
    eq_or_diff(
        scalar(`./deal_freecell 24`),
        <<'EOF',
: 4C 2C 9C 8C QS 4S 2H
: 5H QH 3C AC 3H 4H QD
: QC 9S 6H 9H 3S KS 3D
: 5D 2S JC 5C JH 6D AS
: 2D KD TH TC TD 8D
: 7H JS KH TS KC 7C
: AH 5S 6S AD 8H JD
: 7S 6C 7D 4D 8S 9D
EOF
        "Deal Freecell #24",
    );
}

