export default class CustomDate extends Date {
	public addDays(days: number): CustomDate {
		const date = new CustomDate(this.valueOf());
		date.setDate(date.getDate() + days);
		return date;
	}

	public static format(date: string): string {
		return new CustomDate(date).toLocaleString();
	}
}
