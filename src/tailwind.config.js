const round = (/** @type {number} */ num) =>
	num
		.toFixed(7)
		.replace(/(\.[0-9]+?)0+$/, '$1')
		.replace(/\.0$/, '');
const rem = (/** @type {number} */ px) => `${round(px / 16)}rem`;
const em = (/** @type {number} */ px, /** @type {number} */ base) => `${round(px / base)}em`;
/** @type {import('tailwindcss').Config} */
module.exports = {
	theme: {
		extend: {
			typography: {
				DEFAULT: {
					css: {
						h1: {
							fontWeight: 900,
							fontSize: em(36, 16),
							marginTop: em(32, 36),
							marginBottom: em(16, 36),
							lineHeight: round(40 / 36)
						},
						h2: {
							fontWeight: 700,
							fontSize: em(24, 16),
							marginTop: em(48, 24),
							marginBottom: em(24, 24),
							lineHeight: round(32 / 24)
						},
						h3: {
							fontWeight: 600,
							fontSize: em(20, 16),
							marginTop: em(32, 20),
							marginBottom: em(12, 20),
							lineHeight: round(32 / 20)
						},
						h4: {
							fontWeight: 500,
							marginTop: em(24, 16),
							marginBottom: em(8, 16),
							lineHeight: round(24 / 16)
						},
						pre: {
							gridColumn: 'feature'
						},
						'code::before': {
							content: 'none'
						},
						'code::after': {
							content: 'none'
						},

						blockquote: {
							gridColumn: 'popout',
							background: 'var(--color-secondary)'
						},

						'blockquote p:first-of-type::before': {
							content: 'none'
						},
						'blockquote p:last-of-type::after': {
							content: 'none'
						}
					}
				}
			}
		}
	}
};
