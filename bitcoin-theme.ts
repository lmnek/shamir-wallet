
import type { CustomThemeConfig } from '@skeletonlabs/tw-plugin';

export const bitcoinTheme: CustomThemeConfig = {
    name: 'bitcoin-theme',
    properties: {
		// =~= Theme Properties =~=
		"--theme-font-family-base": `Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji'`,
		"--theme-font-family-heading": `system-ui`,
		"--theme-font-color-base": "0 0 0",
		"--theme-font-color-dark": "255 255 255",
		"--theme-rounded-base": "24px",
		"--theme-rounded-container": "16px",
		"--theme-border-base": "4px",
		// =~= Theme On-X Colors =~=
		"--on-primary": "0 0 0",
		"--on-secondary": "255 255 255",
		"--on-tertiary": "255 255 255",
		"--on-success": "255 255 255",
		"--on-warning": "0 0 0",
		"--on-error": "255 255 255",
		"--on-surface": "255 255 255",
		// =~= Theme Colors  =~=
		// primary | #f7931a 
		"--color-primary-50": "254 239 221", // #feefdd
		"--color-primary-100": "253 233 209", // #fde9d1
		"--color-primary-200": "253 228 198", // #fde4c6
		"--color-primary-300": "252 212 163", // #fcd4a3
		"--color-primary-400": "249 179 95", // #f9b35f
		"--color-primary-500": "247 147 26", // #f7931a
		"--color-primary-600": "222 132 23", // #de8417
		"--color-primary-700": "185 110 20", // #b96e14
		"--color-primary-800": "148 88 16", // #945810
		"--color-primary-900": "121 72 13", // #79480d
		// secondary | #0d579b 
		"--color-secondary-50": "219 230 240", // #dbe6f0
		"--color-secondary-100": "207 221 235", // #cfddeb
		"--color-secondary-200": "195 213 230", // #c3d5e6
		"--color-secondary-300": "158 188 215", // #9ebcd7
		"--color-secondary-400": "86 137 185", // #5689b9
		"--color-secondary-500": "13 87 155", // #0d579b
		"--color-secondary-600": "12 78 140", // #0c4e8c
		"--color-secondary-700": "10 65 116", // #0a4174
		"--color-secondary-800": "8 52 93", // #08345d
		"--color-secondary-900": "6 43 76", // #062b4c
		// tertiary | #132c47 
		"--color-tertiary-50": "220 223 227", // #dcdfe3
		"--color-tertiary-100": "208 213 218", // #d0d5da
		"--color-tertiary-200": "196 202 209", // #c4cad1
		"--color-tertiary-300": "161 171 181", // #a1abb5
		"--color-tertiary-400": "90 107 126", // #5a6b7e
		"--color-tertiary-500": "19 44 71", // #132c47
		"--color-tertiary-600": "17 40 64", // #112840
		"--color-tertiary-700": "14 33 53", // #0e2135
		"--color-tertiary-800": "11 26 43", // #0b1a2b
		"--color-tertiary-900": "9 22 35", // #091623
		// success | #329239 
		"--color-success-50": "224 239 225", // #e0efe1
		"--color-success-100": "214 233 215", // #d6e9d7
		"--color-success-200": "204 228 206", // #cce4ce
		"--color-success-300": "173 211 176", // #add3b0
		"--color-success-400": "112 179 116", // #70b374
		"--color-success-500": "50 146 57", // #329239
		"--color-success-600": "45 131 51", // #2d8333
		"--color-success-700": "38 110 43", // #266e2b
		"--color-success-800": "30 88 34", // #1e5822
		"--color-success-900": "25 72 28", // #19481c
		// warning | #F4D933 
		"--color-warning-50": "253 249 224", // #fdf9e0
		"--color-warning-100": "253 247 214", // #fdf7d6
		"--color-warning-200": "252 246 204", // #fcf6cc
		"--color-warning-300": "251 240 173", // #fbf0ad
		"--color-warning-400": "247 228 112", // #f7e470
		"--color-warning-500": "244 217 51", // #F4D933
		"--color-warning-600": "220 195 46", // #dcc32e
		"--color-warning-700": "183 163 38", // #b7a326
		"--color-warning-800": "146 130 31", // #92821f
		"--color-warning-900": "120 106 25", // #786a19
		// error | #EE3902 
		"--color-error-50": "252 225 217", // #fce1d9
		"--color-error-100": "252 215 204", // #fcd7cc
		"--color-error-200": "251 206 192", // #fbcec0
		"--color-error-300": "248 176 154", // #f8b09a
		"--color-error-400": "243 116 78", // #f3744e
		"--color-error-500": "238 57 2", // #EE3902
		"--color-error-600": "214 51 2", // #d63302
		"--color-error-700": "179 43 2", // #b32b02
		"--color-error-800": "143 34 1", // #8f2201
		"--color-error-900": "117 28 1", // #751c01
		// surface | #4d4d4d 
		"--color-surface-50": "228 228 228", // #e4e4e4
		"--color-surface-100": "219 219 219", // #dbdbdb
		"--color-surface-200": "211 211 211", // #d3d3d3
		"--color-surface-300": "184 184 184", // #b8b8b8
		"--color-surface-400": "130 130 130", // #828282
		"--color-surface-500": "77 77 77", // #4d4d4d
		"--color-surface-600": "69 69 69", // #454545
		"--color-surface-700": "58 58 58", // #3a3a3a
		"--color-surface-800": "46 46 46", // #2e2e2e
		"--color-surface-900": "38 38 38", // #262626
		
	}
}
