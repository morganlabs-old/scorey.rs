import { WebviewWindow } from '@tauri-apps/api/window';
import { v4 as uuid } from 'uuid';

type PopupWindowProps = {
	width: number;
	height: number;
	center: boolean;
};

export function new_popup_window(
	url: string,
	title: string,
	props: PopupWindowProps = {
		width: 500,
		height: 270,
		center: true
	}
) {
	const { width, height, center } = props;
	const id = uuid();
	const webview = new WebviewWindow(id, { url, title, width, height, center });

	webview.once('tauri://error', (e) => {
		alert('Failed to create new window.\nPlease check console for more information.');
		console.error('Failed to create window', e);
	});
	webview.once('tauri://close-requested', () => location.reload());

	return webview;
}
