import Note from "src/models/Note"

export const mapToNote = (result: any[]): Note[] => {
    let notes: Note[] = []
    result.forEach(res => {
        let modified_time = new Date(0)
        modified_time.setUTCSeconds(res.modified_time.secs_since_epoch)
        notes.push(new Note(res.file_name, res.path, res.text, modified_time))
    })
    return notes
}