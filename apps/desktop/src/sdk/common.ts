import * as yup from 'yup';

const PHONE_NUMBER_REGEX = /^\+201[0125][0-9]{8}$/;

export enum PhoneNumberUse {
	Parent = 'parent',
	Student = 'student',
	Home = 'home',
	Other = 'other'
}

export const phoneNumberShape = yup.object({
	use: yup.string().oneOf(Object.values(PhoneNumberUse)).required(),
	number: yup.string().matches(PHONE_NUMBER_REGEX).required()
});

export type PhoneNumber = yup.InferType<typeof phoneNumberShape>;
