import { Component, OnInit } from "@angular/core";
import { invoke } from "@tauri-apps/api/tauri";
import Note from "src/models/Note";
import { FETCH_ALL_NOTES_FROM_CACHE, FETCH_NOTE_CONTENT_FROM_CACHE } from "./consts";

@Component({
  selector: "app-root",
  templateUrl: "./app.component.html",
  styleUrls: ["./app.component.css"],
})
export class AppComponent implements OnInit {
  notes: Note[] = []
  content: string = ""

  ngOnInit(): void {
    this.fetchAllNotes();
  }

  public fetchNoteContent(fileName: string) {
    invoke<string>(FETCH_NOTE_CONTENT_FROM_CACHE, { fileName: fileName }).then((content: string) => {
      this.content = content
    })
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
