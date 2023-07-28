<script>
	import { invoke } from '@tauri-apps/api/tauri';

	let content = 'Loading...';

	invoke('read_cognitio_dir').then((val) => {
		content = val;
	});
</script>

<div>
	{content}
</div>
