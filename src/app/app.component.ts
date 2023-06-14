import { Component, OnInit } from "@angular/core";
import { invoke } from "@tauri-apps/api/tauri";
import Note from "src/models/Note";
import { FETCH_ALL_NOTES_FROM_CACHE, FETCH_NOTE_CONTENT_FROM_CACHE, UPDATE_NOTE_IN_CACHE, WRITE_TO_FILE } from "./consts";

@Component({
  selector: "app-root",
  templateUrl: "./app.component.html",
  styleUrls: ["./app.component.css"],
})
export class AppComponent implements OnInit {
  notes: Note[] = []
  selectedNote: string = ""
  content: string = ""
  showingContent: boolean = false
  creating: boolean = false
  editing: boolean = false
  fileName: string = ""
  fileContent: string = ""

  ngOnInit(): void {
    this.fetchAllNotes()
  }

  fetchNoteContent(fileName: string) {
    invoke<string>(FETCH_NOTE_CONTENT_FROM_CACHE, { fileName: fileName }).then((content: string) => {
      this.selectedNote = fileName
      this.showingContent = true
      this.content = content
    })
  }

  save(): void {
    if (this.creating) {
      let note = this.notes.find(n => n.fileName === this.selectedNote)
      invoke<string>(UPDATE_NOTE_IN_CACHE, { fileName: this.fileName, path: note?.path!, text: this.fileContent }).then(() => {
        this.fetchAllNotes()
        this.creating = false
        this.editing = false
        this.showingContent = false
      })
      return
    }

    if (this.editing) {
      invoke<string>(WRITE_TO_FILE, { text: this.fileContent, fileName: this.fileName }).then(() => {
        this.fetchAllNotes()
        this.fetchNoteContent(this.fileName)
        this.fileContent = ""
        this.fileName = ""
        this.creating = false
        this.editing = false
      })
      return
    }
  }

  edit(): void {
    this.editing = true
    if (this.selectedNote !== "") {
      let note = this.notes.find(n => n.fileName === this.selectedNote)
      this.fileContent = note?.text!
      this.fileName = note?.fileName!
    }
  }

  private fetchAllNotes() {
    invoke<string>(FETCH_ALL_NOTES_FROM_CACHE, {}).then((res: any) => {
      this.notes = this.mapToNote(res)
    })
  }

  private mapToNote(result: any[]): Note[] {
    let notes: Note[] = []
    result.forEach(res => {
      let modified_time = new Date(0)
      modified_time.setUTCSeconds(res.modified_time.secs_since_epoch)
      notes.push(new Note(res.file_name, res.path, res.text, modified_time))
    })

    return notes
  }
}
