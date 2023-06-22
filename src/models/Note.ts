export default class Note {
    fileName!: string;
    path!: string;
    text!: string;
    modified_time!: Date;

    constructor(fileName: string, path: string, text: string, modified_time: Date) {
        this.fileName = fileName;
        this.path = path;
        this.text = text;
        this.modified_time = modified_time;
    }

    public getDate(): string {
        if (this.modified_time === undefined) {
            return "??? {modified_time} is unkown"
        }

        return this.modified_time.getDay() + "/" + this.modified_time.getMonth() + "/" + this.modified_time.getFullYear() + " "
            + this.modified_time.getHours() + ":"
            + this.modified_time.getMinutes()
    }
}
