#!/usr/bin/perl

use strict;
use warnings;

use Test::More tests => 3;

use Test::Differences qw( eq_or_diff );

use Path::Tiny qw( path );

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

{
    # TEST
    eq_or_diff(
        scalar(`./deal_freecell 11982`),
        <<'EOF',
: AH 3D KD JC 6C JD KC
: AS 3H 6H 5D 2C 7D 8D
: 4H QS 5S 5C TH 8H 2S
: AC QC 4D 8C QH 9C 3S
: 2D 8S 9H 9D 6D 2H
: 6S 7H JH TD TC QD
: TS AD 9S KH 4S 4C
: JS KS 3C 7C 7S 5H
EOF
        "Deal Freecell #24",
    );
}

{
    my $got_dir = path("./t/data/got-seq-1-10/");
    $got_dir->remove_tree();
    $got_dir->mkpath();
    system( "./multi_deal_freecell",
        "--suffix", ".board", "--dir", $got_dir, "seq", "1", "10" );

    # TEST
    if (
        eq_or_diff(
            scalar(`diff -u -r ./t/data/expected-seq-1-10/ "$got_dir"`), '',
            "Deal 1 to 10",
        )
        )
    {
        $got_dir->remove_tree();
    }
}
