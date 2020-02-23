<?php
//List dir and map into [date, html link, orig filename]
$cdir = array_map(function($filename) {
	$d=date("Y-m-d h:i:sa", intval(stat("ups/$filename")['mtime']));
	return [$d, '<a href="ups/'.$filename.'">'.$filename.'</a> '.$d.'<br>', $filename];
}, scandir('ups'));
asort($cdir); //now sort, so it's sorted by date
//Now dump out all the HTML links, ignoring all the dot/hidden ones
foreach ($cdir as $key => $finfo) {
	if($finfo[2][0] !== '.'){
		echo $finfo[1];
	}
} 
