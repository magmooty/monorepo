export const PHONE_NUMBER_REGEX = /^\+201[0125][0-9]{8}$/;

export interface InfoForRoot {
	namespaces: { [key: string]: string };
	users: { [key: string]: string };
}

export type CreatePayload<T> = Omit<T, 'id'>;

/**
 * Text filter for Arabic names to normalize them. Will remove التنوين, normalize عبد name prefixes, and filter special Arabic characters (أ, إ, ...etc).
 * Works in O(n + m) time complexity where m is the number of عبد in the name and does not mutate the original string.
 * @param name Name to be filtered
 * @returns
 */
export function nameFilter(name: string, autocomplete = false): string {
	let output = '';

	const dalIndexes = [];

	for (let i = 0; i < name.length; i++) {
		switch (name[i]) {
			case 'أ':
			case 'إ':
			case 'آ': {
				output += 'ا';
				break;
			}
			case 'ة': {
				output += 'ه';
				break;
			}
			case 'ئ': {
				output += 'ء';
				break;
			}
			case 'ي': {
				if (autocomplete) {
					// Normalize for search and autocomplete
					output += 'ى';
				} else {
					if (i === name.length - 1) {
						output += 'ى';
					} else if (name[i + 1] === ' ') {
						output += 'ى ';
						i++;
					} else {
						output += 'ي';
					}
				}
				break;
			}
			case 'د': {
				output += name[i];
				dalIndexes.push(output.length - 1);
				break;
			}
			// Tanween, the characters can look empty, but they are not
			case 'َ':
			case 'ً':
			case 'ِ':
			case 'ٍ':
			case 'ُ':
			case 'ٌ':
			case 'ّ':
				break;
			default: {
				output += name[i];
				break;
			}
		}
	}

	// Do two passes to avoid un-normalized characters
	for (const i of dalIndexes) {
		if (!(output[i - 2] === 'ع' && output[i - 1] === 'ب')) {
			continue;
		}

		// Handle عبد ربه -> should normalize to عبدربه
		if (
			output[i + 1] === ' ' &&
			output[i + 2] === 'ر' &&
			output[i + 3] === 'ب' &&
			output[i + 4] === 'ه'
		) {
			output = output.slice(0, i + 1) + 'ربه' + output.slice(i + 5);
			continue;
		}

		// Handle عبد الاه -> should normalize to عبدالاه
		if (
			output[i + 1] === ' ' &&
			output[i + 2] === 'ا' &&
			output[i + 3] === 'ل' &&
			output[i + 4] === 'ا' &&
			output[i + 5] === 'ه'
		) {
			output = output.slice(0, i + 1) + output.slice(i + 2);
			continue;
		}

		// Leave عبدالاه as is
		// This is the only remarkable edge case because it contains ال after عبد
		if (
			output[i + 1] === 'ا' &&
			output[i + 2] === 'ل' &&
			output[i + 3] === 'ا' &&
			output[i + 4] === 'ه'
		) {
			continue;
		}

		// Added spaces are fine for autocomplete, it will be trimmed
		if (output[i + 1] === 'ا' && output[i + 2] === 'ل') {
			output = output.slice(0, i + 1) + ' ' + output.slice(i + 1);
			continue;
		}
	}

	return output;
}
