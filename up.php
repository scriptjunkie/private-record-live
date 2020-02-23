<?php
if(isset($_FILES['video-blob'])){
	$filename = preg_replace("([^\w \d\-_,;\[\]\(\).])", "", $_FILES['video-blob']['name']);
	$ext = pathinfo($filename, PATHINFO_EXTENSION);
	if($ext === 'webm'){
		echo "ok ups/$filename ".filesize("ups/$filename");
		$fin = fopen($_FILES['video-blob']['tmp_name'], 'rb');
		if($fin !== FALSE){
			$fout = fopen("ups/$filename", "ab");
			if($fout !== FALSE){
				while(true){
					$contents = fread($fin, 1024*1024);
					if($contents === FALSE || strlen($contents) === 0){
						break;
					}
					$res = fwrite($fout, $contents);
					if($res === FALSE || $res === 0){
						break;
					}
				}
				fclose($fout);
			}
			fclose($fin);
		}
	} else {
		echo "Bad $filename";
	}
}
