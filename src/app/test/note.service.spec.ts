import { NoteService } from "../services/note.service"

test('should fetch all notes', async () => {
    let noteService = new NoteService()
    let notes = await noteService.fetchNotes()
})