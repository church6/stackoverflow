#!/usr/bin/perl -w
# @generator          :  template.sh
# @filename           :  rmb.pl
# @author             :  Copyright (C) Church.ZHONG
# @date               :  Thu 11 Aug 2022 01:22:47 PM HKT
# @function           :  automatically do something what you want by perl.
# @see                :  https://www.cpan.org/
# @require            :  Perl v5.30.0 ( perl -v )

use strict;
use warnings;
use utf8;
use English;
use Cwd qw(getcwd);
use File::Spec;
use File::Basename;
use Term::ANSIColor;

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
  my $path_curf = File::Spec->rel2abs($name);
  # print("# file in PATH = $path_curf\n");
  my ( $vol, $dirs, $file ) = File::Spec->splitpath($path_curf);
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

sub rust {
  enter( ( caller(0) )[3] );
  my ( $directory, $release, $debug )  = @_;
  my ( $vol,       $dirs,    $target ) = get_abs_path($directory);
  # print "${vol}, ${dirs}, ${file}\n";
  my $path = $dirs . $target;

  redLog("# target = ${target}\n")   if ( defined ${target} );
  redLog("# release = ${release}\n") if ( defined ${release} );
  redLog("# debug = ${debug}\n")     if ( defined ${debug} );
  chdir ${path};
  # print getcwd(), "\n";

  system("cargo add chrono --quiet");
  system("cargo clean --quiet");
  system("cargo update --quiet");
  system("cargo fmt --quiet");
  system("cargo clippy");
  system("cargo test");
  my ${rust_backtrace} = "";
  if ( defined $debug ) {
    if ( 0 == $debug ) {
      ${rust_backtrace} = "";
    } elsif ( 1 == $debug ) {
      ${rust_backtrace} = "RUST_BACKTRACE=1";
    } elsif ( 2 == $debug ) {
      ${rust_backtrace} = "RUST_BACKTRACE=full";
    } else {
      ${rust_backtrace} = "";
    }
  }
  if ( defined ${release} ) {
    qx(cargo build --release);
    system("${rust_backtrace} cargo run --release");
    my $binary = "${path}/target/release/${target}";
    redLog("# binary = ${binary}\n");
    if ( 0 and -f ${binary} ) {
      system( ${binary} );
    }
  } else {
    qx(cargo build);
    system("${rust_backtrace} cargo run");
    my $binary = "${path}/target/debug/${target}";
    redLog("# binary = ${binary}\n");
    if ( 0 and -f ${binary} ) {
      system( ${binary} );
    }
  }
  leave( ( caller(0) )[3] );
}

# -------------------------------- usage --------------------------------
use Getopt::Long;
Getopt::Long::Configure("no_ignore_case");
my $update  = undef;
my $target  = undef;
my $release = undef;
my $debugl  = undef;
my $version = 0;
my $help    = 0;

sub usage {
  print <<EndOfUsage;
  Usage: $0 [options] [files]
  --update     update rust
  --target     compile the directory
  --release    compile the directory with release/debug mode
  --debugl      compile the directory with various debug level
  --version    Version information
  --help       Help information
EndOfUsage
}

# -------------------------------- usage --------------------------------

# -------------------------------- main --------------------------------
# use Time::HiRes qw( time );
sub main() {
  # my $beginT = time();
  my $opt = GetOptions(
    'update'   => \$update,
    'target=s' => \$target,
    'release'  => \$release,
    'debugl=i' => \$debugl,
    'version'  => \$version,
    'help|?'   => \$help
  );
  usage()                 and exit 0            if ( !$opt );
  usage()                 and exit 0            if ($help);
  print "$PERL_VERSION\n" and exit 0            if ($version);
  qx(rustup self update)  and qx(rustup update) if ( defined $update );
  usage()                 and exit 0            if ( !defined $target or !-d $target );

  rust( $target, $release, $debugl );

  my $elapsed_time = time() - $^T;
  # $^T just like $start_time=time() put it very beginning of perl script.
  print "\n# run time is: $elapsed_time second(s) \n";

  # my $endT = time();
  # printf("%.4f\n", $endT - $beginT);
}

# -------------------------------- exit --------------------------------
main();
exit 0;

# perl -c rmb.pl
# sudo apt install -y perltidy
# http://perltidy.sourceforge.net/tutorial.html
# f="/data/2020/rust/01_runoob/rmb.pl";perltidy -ce -l=128 -i=2 -nbbc -b ${f};
