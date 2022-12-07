#!/usr/bin/perl -w
# @generator          :  template.sh
# @filename           :  ids.pl
# @author             :  Copyright (C) Church.ZHONG
# @date               :  Tue Nov 22 10:12:44 AM HKT 2022
# @function           :  automatically do something what you want by perl.
# @see                :  https://www.cpan.org/
# @require            :  perl (v5.34.0)

use strict;
use warnings;
use utf8;
use File::Spec;
use File::Copy;
use File::Path qw(make_path);
use Term::ANSIColor;
use Carp;

use HTML::Entities;

sub ltrim { my $s = shift; $s =~ s/^\s+//;       return $s }
sub rtrim { my $s = shift; $s =~ s/\s+$//;       return $s }
sub trim  { my $s = shift; $s =~ s/^\s+|\s+$//g; return $s }

sub open_filehandle_for_write {
  my $filename = $_[0];
  local *FH;
  use open ':encoding(UTF-8)';
  open( FH, '>', $filename ) || die "Could not open $filename";
  binmode FH, ":encoding(UTF-8)";
  return *FH;
}

sub open_filehandle_for_read {
  my $filename = $_[0];
  local *FH;
  open( FH, $filename ) || die "Could not open $filename";
  return *FH;
}

sub get_abs_path {
  my $name = shift;
  $name = __FILE__ if ( ( !defined $name ) || ( !-e $name ) );
  # best code, get file true path.
  my ${path_curf} = File::Spec->rel2abs($name);
  # print("# file in PATH = ${path_curf}\n");
  my ( $vol, $dirs, $file ) = File::Spec->splitpath( ${path_curf} );
  # print("# file in Dir = $dirs\n");
  return ( $vol, $dirs, $file );
}

sub enter {
  my $t  = localtime();
  my $fn = shift;
  $fn =~ s/^.*:://;
  print( colored( '[' . $t . '][enter]' . ${fn}, 'green' ), "\n" );
}

sub leave {
  my $t  = localtime();
  my $fn = shift;
  $fn =~ s/^.*:://;
  print( colored( '[' . $t . '][leave]' . ${fn}, 'green' ), "\n" );
}

sub printArray {
  my $name      = shift;
  my $seperator = shift;
  print "${name}:[";
  foreach my $e (@_) {
    $e = 'undef' if !defined $e;
    print "${e}${seperator}";
  }
  print "]\n";
}

sub printHash {
  my $name      = shift;
  my $seperator = shift;
  my $hRef      = shift;
  print "${name}:{";
  foreach my $k ( keys %{$hRef} ) { print "$k => ${$hRef}{$k}${seperator}"; }
  print "}\n";
}

sub redLog    { print colored( @_, 'red' ); }
sub greenLog  { print colored( @_, 'green' ); }
sub blueLog   { print colored( @_, 'blue' ); }
sub yellowLog { print colored( @_, 'yellow' ); }

my ( $vol, $dirs, $dummy ) = get_abs_path();

use Readonly;
Readonly my $DATE       => "Wed Nov 16 12:31:09 PM HKT 2022";
Readonly my $OUTPUT_DIR => "${dirs}/../church";

sub write_example_by_id {
  my ( ${count}, ${id}, ${title} ) = @_;
  #print "${id}, ${title}\n";
  #print "${OUTPUT_DIR}\n";

  my $filename = "stackoverflow_${count}_${id}.rs";
  my $wfh      = open_filehandle_for_write("${OUTPUT_DIR}/src/${filename}");
  print $wfh "// \@filename   : ${filename}
// \@date       : ${DATE}
// \@author     : Church.ZHONG
// \@see        : https://stackoverflow.com/questions/${id}
// \@title      : ${title}

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        pub fn test() {
            // add your code here
        }
    }
    mod code2 {
        pub fn test() {
            // add your code here
        }
    }
    mod code3 {
        pub fn test() {
            // add your code here
        }
    }
    pub fn test() {
        //code1::test();
        //code2::test();
        //code3::test();
    }
}
#[allow(dead_code)]
mod answer2 {
    mod code1 {
        pub fn test() {
            // add your code here
        }
    }
    mod code2 {
        pub fn test() {
            // add your code here
        }
    }
    mod code3 {
        pub fn test() {
            // add your code here
        }
    }
    pub fn test() {
        //code1::test();
        //code2::test();
        //code3::test();
    }
}
#[allow(dead_code)]
mod answer3 {
    mod code1 {
        pub fn test() {
            // add your code here
        }
    }
    mod code2 {
        pub fn test() {
            // add your code here
        }
    }
    mod code3 {
        pub fn test() {
            // add your code here
        }
    }
    pub fn test() {
        //code1::test();
        //code2::test();
        //code3::test();
    }
}
pub fn test() {
    _enter!();
    //answer1::test();
    //answer2::test();
    //answer3::test();
    _leave!();
}
";
  close $wfh;
}

sub write_main {
  my $wfh = shift;

  print $wfh 'fn main() {
    _enter!();
    println!("Hello, world!");
    if cfg!(feature = "html_001") {
        html_001::test();
';

  for my $second ( 2 .. 60 ) {
    my $align = sprintf "%03d", ${second};
    #print "${align}\n";
    print $wfh "    } else if cfg!(feature = \"html_${align}\") {
        html_${align}::test();
";
  }

  print $wfh '    }
    _leave!();
}
';

}

sub write_ids {
  my $ids_href = shift;

  my $filename = "ids_stackoverflow.py";
  my $wfh      = open_filehandle_for_write("${OUTPUT_DIR}/${filename}");
  print $wfh "# pylint: disable=C0301
# pylint: disable=C0302
\'\'\'
# This file is generated automatically by ids.pl
# \@filename           :  ${filename}
# \@author             :  Copyright (C) Church.ZHONG
# \@date               :  ${DATE}
\'\'\'
IDS_STACKOVERFLOW = {
";

  for my $sixty ( 1 .. 60 ) {
    my $align = sprintf "%03d", ${sixty};
    my $ids   = ${ids_href}->{ 'html_' . ${align} . '.html' };

    print $wfh "\"html_${align}\": [";
    print $wfh join( ',', @{ ${ids} } );
    print $wfh "],\n";
  }

  print $wfh "}\n";
  close $wfh;
}

sub work {
  enter( ( caller(0) )[3] );
  #redLog("work\n");
  #greenLog("work\n");
  #blueLog("work\n");
  #yellowLog("work\n");

  make_path( ${OUTPUT_DIR} );

  my $count    = 1;
  my %ids_hash = ();

  my $wfh = open_filehandle_for_write("${OUTPUT_DIR}/src/main.rs");
  print $wfh "#[macro_use]\nmod util;\n";

  for my $sixty ( 1 .. 60 ) {
    my $align = sprintf "%03d", ${sixty};
    print "${dirs}/${align}.html\n";

    my @unordered_ids = ();
    my @ordered_ids   = ();
    my $rfh           = open_filehandle_for_read("${dirs}/${align}.html");
    while (<$rfh>) {
      if (
        m{ \A
\s*<a
\s*href="/questions/([0-9]+)/([0-9a-z\-\%]+)"
\s*class="s-link">(.*)</a>
\n? \z }xms
        )
      {
        # dos2unix
        # my $title = substr( ${3}, 0, -6 );
        # my $title = ${3};
        # print ${1}, ',', ${2}, ',', decode_entities( ${3} ), "\n";
        my $count_3000 = sprintf( "%04d", ${count} );
        write_example_by_id( ${count_3000}, ${1}, decode_entities( ${3} ) );
        ${count} = ${count} + 1;
        push @unordered_ids, ${1};
        push @ordered_ids,   "${count_3000}_${1}";
      }
    }
    close $rfh;

    print "FIXME ${align} and ", scalar(@unordered_ids), "\n" and exit 0 if ( 50 != scalar(@unordered_ids) );
    ${ids_hash}{ 'html_' . ${align} . '.html' } = \@unordered_ids;

    # ----------------------------------------------------------------
    print $wfh "// ${align}.html\n";
    for my $id (@ordered_ids) {
      print $wfh "mod stackoverflow_${id};\n";
    }

    print $wfh "mod html_${align} {
    pub fn test() {
        //_enter!();
";

    for my $id (@ordered_ids) {
      print $wfh "        super::stackoverflow_${id}::test();\n";
    }

    print $wfh '        //_leave!();
    }
}

';
    # ----------------------------------------------------------------
  }
  write_ids( \%ids_hash );

  copy( "${dirs}/../util.rs", "${OUTPUT_DIR}/src/util.rs" ) or die "Copy failed: $!";
  write_main $wfh;
  close $wfh;

  leave( ( caller(0) )[3] );
}

# -------------------------------- usage --------------------------------
use Getopt::Long;
Getopt::Long::Configure("no_ignore_case");
my $loglevel = 0;
my $version  = 0;
my $help     = 0;

sub usage {
  print <<EndOfUsage;
  Usage: $0 [options] [files]
  --loglevel               Set loglevel(0-7)
  --version                Version information
  --help                   Help information
EndOfUsage
}

# -------------------------------- usage --------------------------------

# -------------------------------- main --------------------------------
# use Time::HiRes qw( time );
sub main() {
  # my $beginT = time();
  my $opt = GetOptions(
    'loglevel=i' => \$loglevel,
    'version'    => \$version,
    'help|?'     => \$help
  );
  usage()                         and exit 0 if ( !$opt );
  usage()                         and exit 0 if ($help);
  print "0.0.0.1\n"               and exit 0 if ($version);
  print "loglevel is $loglevel\n" and exit 0 if ($loglevel);
  print "# pwd = ${dirs}\n";

  work();

  my $elapsed_time = time() - $^T;
  # $^T just like $start_time=time() put it very beginning of perl script.
  print "\n# run time is: $elapsed_time second(s) \n";

  # my $endT = time();
  # printf("%.4f\n", $endT - $beginT);
}

# -------------------------------- exit --------------------------------
main();
exit 0;

# perl -c ids.pl

# sudo apt install -y perltidy
# http://perltidy.sourceforge.net/tutorial.html
# f="/data/rust/stackoverflow/questions/ids.pl";perltidy -ce -l=128 -i=2 -nbbc -b ${f};

# https://metacpan.org/pod/Config::Tiny
#CPAN shell:
# sudo perl -MCPAN -e shell
# install Config::Tiny

