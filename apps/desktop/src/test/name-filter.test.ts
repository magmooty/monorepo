import { nameFilter } from 'sdk/common';
import { describe, expect, it } from 'vitest';

describe('Text Filter', () => {
	it('should filter special arabic characters', () => {
		expect(nameFilter('أحمد')).toEqual('احمد');
		expect(nameFilter('إيمان')).toEqual('ايمان');
		expect(nameFilter('عزة')).toEqual('عزه');
		expect(nameFilter('روميسائ')).toEqual('روميساء');
		expect(nameFilter('آيات')).toEqual('ايات');
		expect(nameFilter('علي')).toEqual('على');
		expect(nameFilter('أحمد محمد علي')).toEqual('احمد محمد على');
	});

	it('should normalize Abd prefixes', () => {
		expect(nameFilter('عبدالرحمن')).toEqual('عبد الرحمن');
		expect(nameFilter('عبدالله')).toEqual('عبد الله');
		expect(nameFilter('عبد الملك')).toEqual('عبد الملك');
		expect(nameFilter('عبد ربه')).toEqual('عبدربه');
		expect(nameFilter('عبد الاه')).toEqual('عبدالاه');
	});

	it('should not normalize valid names with Abd prefixes', () => {
		expect(nameFilter('عبدون')).toEqual('عبدون');
		expect(nameFilter('عبده')).toEqual('عبده');
		expect(nameFilter('عبدربه')).toEqual('عبدربه');
		expect(nameFilter('عبدالاه')).toEqual('عبدالاه');
	});

	it('should remove tanween', () => {
		expect(nameFilter('كٍتًاَبٍه')).toEqual('كتابه');
	});

	it('should not normalize single letter', () => {
		expect(nameFilter('ز')).toEqual('ز');
	});

	it('should normalize ي when preparing for autocomplete', () => {
		expect(nameFilter('ي', true)).toEqual('ى');
		expect(nameFilter('زي', true)).toEqual('زى');
		expect(nameFilter('زيا', true)).toEqual('زىا');
		expect(nameFilter('علي', true)).toEqual('على');
		expect(nameFilter('على', true)).toEqual('على');
	});
});
