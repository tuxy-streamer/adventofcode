package day5;
use Exporter 'import';
our @EXPORT_OK = qw(run);

use strict;
use warnings;

use File::Slurp::Tiny;

sub first_half {
	my (@data) = @_;
	my $no_of_nice_string = 0;

	foreach my $line ( @data ) {
		my $vowel_counter = () = $line =~ /[aeiou]/g;
		my $has_doubles = ($line =~ /(.)\1/);
		my $has_unwanted_chars = ($line =~ /ab|cd|pq|xy/);

		if (
			$vowel_counter >= 3 &&
			$has_doubles &&
			!$has_unwanted_chars
		) {
			$no_of_nice_string++;
		}
	}
	return $no_of_nice_string;
}

sub second_half {
    my (@data) = @_;
    my $no_of_nice_string = 0;

    foreach my $line (@data) {
        chomp $line;
        my $has_pair_twice = ($line =~ /(..).*\1/);
        my $has_repeat_with_gap = ($line =~ /(.).\1/);

        if ($has_pair_twice && $has_repeat_with_gap) {
            $no_of_nice_string++;
        }
    }

    return $no_of_nice_string;
}

sub run {
	my @data = File::Slurp::Tiny::read_lines("2015/day5/day5.txt");
	my $no_of_nice_string = first_half(@data);
	print "First Half (no_of_nice_string) : $no_of_nice_string\n";
	$no_of_nice_string = second_half(@data);
	print "Second Half (no_of_nice_string) : $no_of_nice_string\n";
}

1;
