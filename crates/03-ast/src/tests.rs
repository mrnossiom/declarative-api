use crate::types::*;
use lexer::{span::Span, symbols::Symbol};

#[allow(clippy::too_many_lines)]
fn _dummy_api() -> Api {
	Api {
		meta: vec![
			Attribute {
				ident: Ident {
					symbol: Symbol::new_static("doc"),
					span: Span { start: 0, end: 0 },
				},
				value: " This Api is a test for the documentation generator of the api".into(),
				span: Span { start: 0, end: 0 },
			},
			Attribute {
				ident: Ident {
					symbol: Symbol::new_static("doc"),
					span: Span { start: 0, end: 0 },
				},
				value: " This is a second line of comment".into(),
				span: Span { start: 0, end: 0 },
			},
			Attribute {
				ident: Ident {
					symbol: Symbol::new_static("doc"),
					span: Span { start: 0, end: 0 },
				},
				value: " This is a third line of comment".into(),
				span: Span { start: 0, end: 0 },
			},
		],
		items: vec![
			Item {
				attrs: vec![],
				kind: ItemKind::Meta(Meta {
					fields: vec![
						MetaField {
							ident: Ident {
								symbol: Symbol::new_static("name"),
								span: Span { start: 0, end: 0 },
							},
							value: MetaFieldKind::Str("Wiro's API".into()),
							span: Span { start: 0, end: 0 },
						},
						MetaField {
							ident: Ident {
								symbol: Symbol::new_static("description"),
								span: Span { start: 0, end: 0 },
							},
							value: MetaFieldKind::Str("This is the API of Wiro".into()),
							span: Span { start: 0, end: 0 },
						},
						MetaField {
							ident: Ident {
								symbol: Symbol::new_static("version"),
								span: Span { start: 0, end: 0 },
							},
							value: MetaFieldKind::Str("1.0.0".into()),
							span: Span { start: 0, end: 0 },
						},
						MetaField {
							ident: Ident {
								symbol: Symbol::new_static("name"),
								span: Span { start: 0, end: 0 },
							},
							value: MetaFieldKind::Vec(vec![
								MetaFieldKind::Str("https://paradigm.lighton.ai/api/v1".into()),
								MetaFieldKind::Str(
									"https://paradigm-preprod.lighton.ai/api/v1".into(),
								),
								MetaFieldKind::Str("https://paradigm-dev.lighton.ai/api/v1".into()),
							]),
							span: Span { start: 0, end: 0 },
						},
					],
				}),
				span: Span { start: 0, end: 0 },
				ident: Ident {
					symbol: Symbol::new_static(""),
					span: Span { start: 0, end: 0 },
				},
			},
			Item {
				attrs: vec![Attribute {
					ident: Ident {
						symbol: Symbol::new_static("doc"),
						span: Span { start: 0, end: 0 },
					},
					value: " Imports the `builder.dapi` file".into(),
					span: Span { start: 0, end: 0 },
				}],
				ident: Ident {
					symbol: Symbol::new_static("builder"),
					span: Span { start: 0, end: 0 },
				},
				kind: ItemKind::Scope(ScopeKind::Unloaded),
				span: Span { start: 0, end: 0 },
			},
			Item {
				attrs: vec![],
				ident: Ident {
					symbol: Symbol::new_static("dashboard"),
					span: Span { start: 0, end: 0 },
				},
				kind: ItemKind::Scope(ScopeKind::Loaded {
					items: vec![Item {
						attrs: vec![],
						ident: Ident {
							symbol: Symbol::new_static("dashboard"),
							span: Span { start: 0, end: 0 },
						},
						kind: ItemKind::Path(Path {
							path: PathKind::String(Ident {
								symbol: Symbol::new_static("dashboard"),
								span: Span { start: 0, end: 0 },
							}),
							items: vec![Item {
								attrs: vec![],
								ident: Ident {
									symbol: Symbol::new_static(""),
									span: Span { start: 0, end: 0 },
								},
								kind: ItemKind::Headers(Headers {
									headers: vec![
										HeaderField {
											attrs: vec![
												Attribute {
													ident: Ident {
														symbol: Symbol::new_static("doc"),
														span: Span { start: 0, end: 0 },
													},
													value: " # Safety".into(),
													span: Span { start: 0, end: 0 },
												},
												Attribute {
													ident: Ident {
														symbol: Symbol::new_static("doc"),
														span: Span { start: 0, end: 0 },
													},
													value: " This is a comment".into(),
													span: Span { start: 0, end: 0 },
												},
												Attribute {
													ident: Ident {
														symbol: Symbol::new_static("doc"),
														span: Span { start: 0, end: 0 },
													},
													value: " This is a second line of comment"
														.into(),
													span: Span { start: 0, end: 0 },
												},
												Attribute {
													ident: Ident {
														symbol: Symbol::new_static("description"),
														span: Span { start: 0, end: 0 },
													},
													value: "The API Key of the User".into(),
													span: Span { start: 0, end: 0 },
												},
												Attribute {
													ident: Ident {
														symbol: Symbol::new_static("prefix"),
														span: Span { start: 0, end: 0 },
													},
													value: "Api-Key".into(),
													span: Span { start: 0, end: 0 },
												},
											],
											ident: Ident {
												symbol: Symbol::new_static("Authorization"),
												span: Span { start: 0, end: 0 },
											},
											span: Span { start: 0, end: 0 },
										},
										HeaderField {
											attrs: vec![Attribute {
												ident: Ident {
													symbol: Symbol::new_static("description"),
													span: Span { start: 0, end: 0 },
												},
												value: "The Model of the User".into(),
												span: Span { start: 0, end: 0 },
											}],
											ident: Ident {
												symbol: Symbol::new_static("X-Model"),
												span: Span { start: 0, end: 0 },
											},
											span: Span { start: 0, end: 0 },
										},
									],
								}),
								span: Span { start: 0, end: 0 },
							}],
						}),
						span: Span { start: 0, end: 0 },
					}],
					span: Span { start: 0, end: 0 },
				}),
				span: Span { start: 0, end: 0 },
			},
		],
		span: Span { start: 0, end: 0 },
	}
}
