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
Readonly my $DATE       => "Fri Nov 25 09:09:59 AM HKT 2022";
Readonly my $OUTPUT_DIR => "${dirs}/../church";

sub write_0 {
  my $filename = 'colors.py';
  my $wfh      = open_filehandle_for_write("${OUTPUT_DIR}/${filename}");
  print $wfh "\'\'\'
# \@filename     :  ${filename}
# \@author       :  Copyright (C) Church.Zhong
# \@date         :  ${DATE}
# \@title        :
# \@see          :  https://docs.python.org/3/library/datetime.html
# \@require      :  Python 3.10.6 works well.
# \@style        :  https://google.github.io/styleguide/pyguide.html
\'\'\'

FOREGROUND_BLACK = '\033[30m'
FOREGROUND_RED = '\033[31m'
FOREGROUND_GREEN = '\033[32m'
FOREGROUND_YELLOW = '\033[33m'
FOREGROUND_BLUE = '\033[34m'
FOREGROUND_MAGENTA = '\033[35m'
FOREGROUND_CYAN = '\033[36m'
FOREGROUND_LIGHT_GRAY = '\033[37m'
FOREGROUND_GRAY = '\033[90m'
FOREGROUND_LIGHT_RED = '\033[91m'
FOREGROUND_LIGHT_GREEN = '\033[92m'
FOREGROUND_LIGHT_YELLOW = '\033[93m'
FOREGROUND_LIGHT_BLUE = '\033[94m'
FOREGROUND_LIGHT_MAGENTA = '\033[95m'
FOREGROUND_LIGHT_CYAN = '\033[96m'
ENDCOLOR = '\033[0m'
LINE = '-' * 64
";
  close $wfh;
}

sub write_1 {
  my $filename = 'main.py';
  my $wfh      = open_filehandle_for_write("${OUTPUT_DIR}/${filename}");
  print $wfh "#!/usr/bin/python3
\'\'\'
# \@filename     :  ${filename}
# \@author       :  Copyright (C) Church.Zhong
# \@date         :  ${DATE}
# \@title        :
# \@see          :  https://docs.python.org/3/library/datetime.html
# \@require      :  Python 3.10.6 works well.
# \@style        :  https://google.github.io/styleguide/pyguide.html
\'\'\'

# import os
import sys
import time

# import re
# import random
import argparse

from ids_stackoverflow import IDS_STACKOVERFLOW


def work():
    '''
    Description : work
    '''
    parser = argparse.ArgumentParser()
    parser.add_argument('--id', '-i', type=int, action='store', default=None, dest='id', help='id')
    results = parser.parse_args()

    if results.id is None:
        sys.exit(0)

    stackoverflow_id = results.id
    count = 1
    for key, value in IDS_STACKOVERFLOW.items():
        if stackoverflow_id in value:
            print(key)
            print(stackoverflow_id)
            page = int(key.split('_')[1])
            order = value.index(stackoverflow_id)
            count = count + (page - 1) * 50 + order
            answers = __import__(f'stackoverflow_{count:>04d}_{stackoverflow_id}')
            answers.verify()


def main():
    '''
    Description : main
    '''
    start = time.time()
    work()
    print(f'# elapsed time:{time.time() - start}')


if __name__ == \"__main__\":
    main()
";
  close $wfh;
}

sub write_example_by_id {
  my ( ${count}, ${id}, ${title} ) = @_;
  #print "${id}, ${title}\n";
  #print "${OUTPUT_DIR}\n";

  my $filename = "stackoverflow_${count}_${id}.py";
  my $wfh      = open_filehandle_for_write("${OUTPUT_DIR}/${filename}");
  print $wfh "#!/usr/bin/python3
# pylint: disable=C0209
\'\'\'
# \@filename     :  ${filename}
# \@author       :  Copyright (C) Church.Zhong
# \@date         :  ${DATE}
# \@title        :  ${title}
# \@see          :  https://docs.python.org/3/library/datetime.html
# \@require      :  Python 3.10.6 works well.
# \@style        :  https://google.github.io/styleguide/pyguide.html
\'\'\'

from colors import FOREGROUND_RED, FOREGROUND_GREEN, FOREGROUND_BLUE, FOREGROUND_YELLOW, ENDCOLOR, LINE


class Answer1:
    '''
    Description : Answer1
    '''

    \@staticmethod
    def code1():
        '''
        Description : code1
        '''
        print('Answer1::code1')

    \@staticmethod
    def code2():
        '''
        Description : code2
        '''
        print('Answer1::code2')

    \@staticmethod
    def code3():
        '''
        Description : code3
        '''
        print('Answer1::code3')

    \@staticmethod
    def verify():
        '''
        Description : verify
        '''
        print(f'{FOREGROUND_RED}{LINE}{ENDCOLOR}')
        Answer1.code1()
        Answer1.code2()
        Answer1.code3()
        print(f'{FOREGROUND_RED}{LINE}{ENDCOLOR}')


class Answer2:
    '''
    Description : Answer2
    '''

    \@staticmethod
    def code1():
        '''
        Description : code1
        '''
        print('Answer2::code1')

    \@staticmethod
    def code2():
        '''
        Description : code2
        '''
        print('Answer2::code2')

    \@staticmethod
    def code3():
        '''
        Description : code3
        '''
        print('Answer2::code3')

    \@staticmethod
    def verify():
        '''
        Description : verify
        '''
        print(f'{FOREGROUND_GREEN}{LINE}{ENDCOLOR}')
        Answer2.code1()
        Answer2.code2()
        Answer2.code3()
        print(f'{FOREGROUND_GREEN}{LINE}{ENDCOLOR}')


class Answer3:
    '''
    Description : Answer3
    '''

    \@staticmethod
    def code1():
        '''
        Description : code1
        '''
        print('Answer3::code1')

    \@staticmethod
    def code2():
        '''
        Description : code2
        '''
        print('Answer3::code2')

    \@staticmethod
    def code3():
        '''
        Description : code3
        '''
        print('Answer3::code3')

    \@staticmethod
    def verify():
        '''
        Description : verify
        '''
        print(f'{FOREGROUND_BLUE}{LINE}{ENDCOLOR}')
        Answer3.code1()
        Answer3.code2()
        Answer3.code3()
        print(f'{FOREGROUND_BLUE}{LINE}{ENDCOLOR}')


class Answer4:
    '''
    Description : Answer4
    '''

    \@staticmethod
    def code1():
        '''
        Description : code1
        '''
        print('Answer4::code1')

    \@staticmethod
    def code2():
        '''
        Description : code2
        '''
        print('Answer4::code2')

    \@staticmethod
    def code3():
        '''
        Description : code3
        '''
        print('Answer4::code3')

    \@staticmethod
    def verify():
        '''
        Description : verify
        '''
        print(f'{FOREGROUND_YELLOW}{LINE}{ENDCOLOR}')
        Answer4.code1()
        Answer4.code2()
        Answer4.code3()
        print(f'{FOREGROUND_YELLOW}{LINE}{ENDCOLOR}')


def verify():
    '''
    Description : verify
    '''
    # Answer1.verify()
    # Answer2.verify()
    # Answer3.verify()
    # Answer4.verify()
";
  close $wfh;
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
  for my $sixty ( 1 .. 60 ) {
    my $align = sprintf "%03d", ${sixty};
    print "${dirs}/${align}.html\n";

    my @unordered_ids = ();
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
      }
    }
    close $rfh;

    print "FIXME ${align} and ", scalar(@unordered_ids), "\n" and exit 0 if ( 50 != scalar(@unordered_ids) );
    ${ids_hash}{ 'html_' . ${align} . '.html' } = \@unordered_ids;
  }

  write_ids( \%ids_hash );

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

