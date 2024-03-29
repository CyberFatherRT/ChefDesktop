import tippy, { type Props } from "tippy.js";
import "tippy.js/dist/tippy.css";

type Options = Partial<Props>;

export function tooltip(element: HTMLElement, options: Options) {
	const option: Options = {
		...options,
		arrow: false,
		theme: "custom"
	};

	const tooltip = tippy(element, option);

	return {
		update() {
			tooltip.setProps(option);
		},
		destroy() {
			tooltip.destroy();
		}
	};
}
