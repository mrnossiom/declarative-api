use crate::ast::*;

pub fn dummy_api() -> Api {
	Api {
		attrs: vec![
			Attribute {
				ident: Ident {
					symbol: Symbol("doc".into()),
					span: SpanData { start: 0, end: 0 },
				},
				value: " This Api is a test for the documentation generator of the api".into(),
				span: SpanData { start: 0, end: 0 },
			},
			Attribute {
				ident: Ident {
					symbol: Symbol("doc".into()),
					span: SpanData { start: 0, end: 0 },
				},
				value: " This is a second line of comment".into(),
				span: SpanData { start: 0, end: 0 },
			},
			Attribute {
				ident: Ident {
					symbol: Symbol("doc".into()),
					span: SpanData { start: 0, end: 0 },
				},
				value: " This is a third line of comment".into(),
				span: SpanData { start: 0, end: 0 },
			},
		],
		items: vec![
			Item {
				attrs: vec![],
				kind: ItemKind::Meta(Meta {
					fields: vec![
						MetaField {
							ident: Ident {
								symbol: Symbol("name".into()),
								span: SpanData { start: 0, end: 0 },
							},
							value: MetaFieldKind::Str("Wiro's API".into()),
							span: SpanData { start: 0, end: 0 },
						},
						MetaField {
							ident: Ident {
								symbol: Symbol("description".into()),
								span: SpanData { start: 0, end: 0 },
							},
							value: MetaFieldKind::Str("This is the API of Wiro".into()),
							span: SpanData { start: 0, end: 0 },
						},
						MetaField {
							ident: Ident {
								symbol: Symbol("version".into()),
								span: SpanData { start: 0, end: 0 },
							},
							value: MetaFieldKind::Str("1.0.0".into()),
							span: SpanData { start: 0, end: 0 },
						},
						MetaField {
							ident: Ident {
								symbol: Symbol("name".into()),
								span: SpanData { start: 0, end: 0 },
							},
							value: MetaFieldKind::Vec(vec![
								MetaFieldKind::Str("https://paradigm.lighton.ai/api/v1".into()),
								MetaFieldKind::Str(
									"https://paradigm-preprod.lighton.ai/api/v1".into(),
								),
								MetaFieldKind::Str("https://paradigm-dev.lighton.ai/api/v1".into()),
							]),
							span: SpanData { start: 0, end: 0 },
						},
					],
				}),
				span: SpanData { start: 0, end: 0 },
				ident: Ident {
					symbol: Symbol("".into()),
					span: SpanData { start: 0, end: 0 },
				},
				id: NodeId(0),
			},
			Item {
				attrs: vec![Attribute {
					ident: Ident {
						symbol: Symbol("doc".into()),
						span: SpanData { start: 0, end: 0 },
					},
					value: " Imports the `builder.dapi` file".into(),
					span: SpanData { start: 0, end: 0 },
				}],
				ident: Ident {
					symbol: Symbol("builder".into()),
					span: SpanData { start: 0, end: 0 },
				},
				kind: ItemKind::Scope(ScopeKind::Unloaded),
				span: SpanData { start: 0, end: 0 },
				id: NodeId(0),
			},
			Item {
				attrs: vec![],
				ident: Ident {
					symbol: Symbol("dashboard".into()),
					span: SpanData { start: 0, end: 0 },
				},
				kind: ItemKind::Scope(ScopeKind::Loaded {
					items: vec![Item {
						attrs: vec![],
						ident: Ident {
							symbol: Symbol("dashboard".into()),
							span: SpanData { start: 0, end: 0 },
						},
						kind: ItemKind::Path(Path {
							path: PathKind::String(Ident {
								symbol: Symbol("dashboard".into()),
								span: SpanData { start: 0, end: 0 },
							}),
							items: vec![Item {
								attrs: vec![],
								ident: Ident {
									symbol: Symbol("".into()),
									span: SpanData { start: 0, end: 0 },
								},
								kind: ItemKind::Headers(Headers {
									headers: vec![
										HeaderField {
											attrs: vec![
												Attribute {
													ident: Ident {
														symbol: Symbol("doc".into()),
														span: SpanData { start: 0, end: 0 },
													},
													value: " # Safety".into(),
													span: SpanData { start: 0, end: 0 },
												},
												Attribute {
													ident: Ident {
														symbol: Symbol("doc".into()),
														span: SpanData { start: 0, end: 0 },
													},
													value: " This is a comment".into(),
													span: SpanData { start: 0, end: 0 },
												},
												Attribute {
													ident: Ident {
														symbol: Symbol("doc".into()),
														span: SpanData { start: 0, end: 0 },
													},
													value: " This is a second line of comment"
														.into(),
													span: SpanData { start: 0, end: 0 },
												},
												Attribute {
													ident: Ident {
														symbol: Symbol("description".into()),
														span: SpanData { start: 0, end: 0 },
													},
													value: "The API Key of the User".into(),
													span: SpanData { start: 0, end: 0 },
												},
												Attribute {
													ident: Ident {
														symbol: Symbol("prefix".into()),
														span: SpanData { start: 0, end: 0 },
													},
													value: "Api-Key".into(),
													span: SpanData { start: 0, end: 0 },
												},
											],
											ident: Ident {
												symbol: Symbol("Authorization".into()),
												span: SpanData { start: 0, end: 0 },
											},
											ty: Type("long_string".into()),
											span: SpanData { start: 0, end: 0 },
										},
										HeaderField {
											attrs: vec![Attribute {
												ident: Ident {
													symbol: Symbol("description".into()),
													span: SpanData { start: 0, end: 0 },
												},
												value: "The Model of the User".into(),
												span: SpanData { start: 0, end: 0 },
											}],
											ident: Ident {
												symbol: Symbol("X-Model".into()),
												span: SpanData { start: 0, end: 0 },
											},
											ty: Type("string".into()),
											span: SpanData { start: 0, end: 0 },
										},
									],
								}),
								span: SpanData { start: 0, end: 0 },
								id: NodeId(0),
							}],
						}),
						span: SpanData { start: 0, end: 0 },
						id: NodeId(0),
					}],
					span: SpanData { start: 0, end: 0 },
				}),
				span: SpanData { start: 0, end: 0 },
				id: NodeId(0),
			},
		],
		span: SpanData { start: 0, end: 0 },
		id: NodeId(0),
	}
}
