<?php

function getTotal($array) {
	$sum = 0;
	ksort($array);

	$firstKey = array_key_first($array);
	$lastKey = array_key_last($array);

	if (isset($array[$firstKey]) || isset($array[$lastKey])) {
		$sum = $array[$firstKey] . $array[$lastKey];
	}

	return $sum;
}

$input = fopen("../input.txt", "r") or die("Unable to open file!");
$search = [
	1 => ["1", "one"],
	2 => ["2", "two"],
	3 => ["3", "three"],
	4 => ["4", "four"],
	5 => ["5", "five"],
	6 => ["6", "six"],
	7 => ["7", "seven"],
	8 => ["8", "eight"],
	9 => ["9", "nine"],
];
$one_star_result = 0;
$two_star_result = 0;

while(!feof($input)) {
	$line = fgets($input);
	$one_star_array = [];
	$two_star_array = [];

	foreach ($search as $key => $value) {
		foreach ($value as $search_key => $search_value) {
			$first = strpos($line, $search_value);
			$last = strrpos($line, $search_value);
			if ($first !== false) {
				if ($search_key == 0) {
					$one_star_array[$first] = $key;
				}
				$two_star_array[$first] = $key;
			}
			if ($last !== false) {
				if ($search_key == 0) {
					$one_star_array[$last] = $key;
				}
				$two_star_array[$last] = $key;
			}
		}
	}

	$one_star_total = getTotal($one_star_array);
	$one_star_result += $one_star_total;

	$two_star_total = getTotal($two_star_array);
	$two_star_result += $two_star_total;
}

fclose($input);

echo 'One star result: ' . $one_star_result;
echo "\n";
echo 'Two star result: ' . $two_star_result;

?>