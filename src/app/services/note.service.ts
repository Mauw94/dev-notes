import { Injectable } from "@angular/core";
import { invoke } from "@tauri-apps/api";
import { FETCH_ALL_NOTES_FROM_CACHE } from "../consts";
import { mapToNote } from "../mappers/note.mapper";
import Note from "src/models/Note";

@Injectable({ providedIn: 'root' })
export class NoteService {

    public async fetchNotes(): Promise<Note[]> {
        let notes: Note[] = []
        await invoke<string>(FETCH_ALL_NOTES_FROM_CACHE, {}).then((res: any) => {
            notes = mapToNote(res)
        })
        return notes
    }
}