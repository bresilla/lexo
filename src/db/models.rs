pub mod schema{
    table! {
        bibtexs (uuid) {
            uuid -> Text,
            author -> Nullable<Text>,
            title -> Nullable<Text>,
            year -> Nullable<Text>,
            address -> Nullable<Text>,
            annote -> Nullable<Text>,
            booktitle -> Nullable<Text>,
            chapter -> Nullable<Text>,
            email -> Nullable<Text>,
            edition -> Nullable<Text>,
            editor -> Nullable<Text>,
            howpublished -> Nullable<Text>,
            institution -> Nullable<Text>,
            journal -> Nullable<Text>,
            month -> Nullable<Text>,
            note -> Nullable<Text>,
            number -> Nullable<Text>,
            organization -> Nullable<Text>,
            pages -> Nullable<Text>,
            publisher -> Nullable<Text>,
            school -> Nullable<Text>,
            series -> Nullable<Text>,
            typee -> Nullable<Text>,
            volume -> Nullable<Text>,
            crossref -> Nullable<Text>,
            doi -> Nullable<Text>,
            issn -> Nullable<Text>,
            isbn -> Nullable<Text>,
            url -> Nullable<Text>,
            abstractt -> Nullable<Text>,
            keywords -> Nullable<Text>,
            language -> Nullable<Text>,
        }
    }
}

use schema::bibtexs;
#[derive(Queryable)]
struct BibTex {
    uuid: Option<String>, // The uuid field is used to store a unique identifier for the entry. It is not used by the standard bibliography styles, but may be used by others that produce an annotated bibliography.
    //mandatory
    author: Option<String>, // In the `author`-field, you can specify all contributors to the work you want to cite. BibTeX has several ways of specifying author names but expects in all cases a consistent and exact arrangement so that inaccurate information could lead to the unwanted output of the author's name.
    title: Option<String>, // The title field is saved for the title of a work to be cited. LaTeX-specific writing, such as capitalisation, should be taken into account.
    year: Option<String>,  // The year of publication, or creation if unpublished.
    //not mandatory
    address: Option<String>, //The address-field contains the publisher's (or any other institution) address.
    annote: Option<String>,  // BibTeX Field type
    booktitle: Option<String>, //To cite an `incollection, i.e., a part of a book/collection with its own title, or to give a title to an article, part of an inproceedings, the booktitle-field is used to distinguish. At the same time, title contains the title of the book, or incproceedings`.
    chapter: Option<String>,   // BibTeX field type: chapter
    email: Option<String>, // Email includes the email address of the stated authors if you want to indicate them. It's not a common field and might not be supported.
    edition: Option<String>, // The edition of a book, e.g. "edition = " Entering an ordinal number is recommended by default, and the first letter is usually capitalised.
    editor: Option<String>, // In the `editor`-field, you can specify all editors to the work you want to cite. BibTeX has several ways of specifying editor names but expects in all cases a consistent and exact arrangement so that inaccurate information could lead to the unwanted output of the editor's name.
    howpublished: Option<String>, // The howpublished field is used with the @misc entry, typically as a workaround for a source type BibTeX does not support. The most common example is when citing web pages and specifying a URL.
    institution: Option<String>, // The institution field is used together with the @techreport entry to specify the name of the institution that issued the report.
    journal: Option<String>,     // The name of the journal in which the journal was published.
    month: Option<String>,       // The month of publication, or creation if unpublished.
    note: Option<String>, // The notes field can be used to store additional information about a particular BibTeX entry that may be relevant to the reader or that is needed for a particular entry type and citation style but does not fit in any other field.
    number: Option<String>, // Usually the issue number (or similar identifier) of a journal, magazine or the number of technical report.
    organization: Option<String>, // Usually, an institution, the name of the conference sponsor/organizer is used here.
    pages: Option<String>, // Page range of, for example, a book. You can separate these either with commas or double dashes.
    publisher: Option<String>, // The name of the publisher of the book being cited.
    school: Option<String>, // school is specified when a phdthesis or mastersthesis is cited and is the educational institution where the student wrote the dissertation. This field is required in both entry types.
    series: Option<String>, // The field can, for example, optionally be used to specify the book series in which the book (or similar) was published.
    typee: Option<String>, // Within the BibTeX entry, the type field is used to declare an explicit type, i.e. for technical report "Research Note", or "Government Report".
    volume: Option<String>, //The 'volume' field is used to enter the volume of a journal or multi-volume book.
    //links
    crossref: Option<String>, // If you have several entries referring to the same proceedings, you can use crossref to specify fields common to several entries only once by defining crossref = . Where citationkey is the citation key of the cross-referenced entry.
    doi: Option<String>, // `doi is used to store any DOI (Digital Object Identifier of the International DOI Foundation). DOI is a digital identifier that is intended to be permanent and unique. It is used, among other things, and mostly for articles from scientific journals. It can be used in article, but more rarely also in chapter, book, or inproceedings / conference`. This field is optional in all cases.
    issn: Option<String>, // The issn field is used to store the ISSN (International Standard Serial Number) of a journal. It is used in article, but more rarely also in book, inproceedings / conference, manual, mastersthesis, phdthesis, techreport, unpublished. This field is optional in all cases.
    isbn: Option<String>, // The isbn field is used to store the ISBN (International Standard Book Number) of a book. It is used in book, but more rarely also in article, inproceedings / conference, manual, mastersthesis, phdthesis, techreport, unpublished. This field is optional in all cases.
    url: Option<String>, // The url field is used to store the URL (Uniform Resource Locator), a link to the online version of the work. It is used in article, but more rarely also in book, inproceedings / conference, manual, mastersthesis, phdthesis, techreport, unpublished. This field is optional in all cases.
    //attitional
    abstractt: Option<String>, // The abstract field is used to store the abstract of a work, which is a summary of the work. It is used in article, but more rarely also in book, inproceedings / conference, manual, mastersthesis, phdthesis, techreport, unpublished. This field is optional in all cases.
    keywords: Option<String>, // The keywords field is used to store keywords of a work. It is used in article, but more rarely also in book, inproceedings / conference, manual, mastersthesis, phdthesis, techreport, unpublished. This field is optional in all cases.
    language: Option<String>, // The language field is used to store the language of a work. It is used in article, but more rarely also in book, inproceedings / conference, manual, mastersthesis, phdthesis, techreport, unpublished. This field is optional in all cases.
}