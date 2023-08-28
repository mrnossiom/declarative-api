use ast::{types::*, P};
use lexer::{span::Span, symbols::Symbol};
use thin_vec::thin_vec;

macro_rules! sym {
	($lit:literal) => {
		Symbol::intern($lit)
	};
}

#[allow(clippy::too_many_lines)]
fn _paradigm_example_ast() -> Api {
	Api {
		meta: Metadata {},
		attrs: thin_vec![
			Attribute {
				kind: AttrKind::DocComment(sym!(
					" This Api is a test for the documentation generator of the api"
				)),
				style: AttrStyle::Inner,
				id: AttrId::make_id(),
				span: Span::DUMMY,
			},
			Attribute {
				kind: AttrKind::DocComment(sym!(" This is a second line of comment")),
				style: AttrStyle::Inner,
				id: AttrId::make_id(),
				span: Span::DUMMY,
			},
			Attribute {
				kind: AttrKind::DocComment(sym!(" This is a third line of comment")),
				style: AttrStyle::Inner,
				id: AttrId::make_id(),
				span: Span::DUMMY,
			},
		],
		items: vec![
			P::<Item>::new(Item {
				attrs: thin_vec![],
				kind: ItemKind::Meta(Meta {
					fields: vec![
						MetaField {
							ident: Ident {
								symbol: sym!("name"),
								span: Span::DUMMY,
							},
							value: MetaFieldKind::Str("Wiro's API".into()),
							span: Span::DUMMY,
						},
						MetaField {
							ident: Ident {
								symbol: sym!("description"),
								span: Span::DUMMY,
							},
							value: MetaFieldKind::Str("This is the API of Wiro".into()),
							span: Span::DUMMY,
						},
						MetaField {
							ident: Ident {
								symbol: sym!("version"),
								span: Span::DUMMY,
							},
							value: MetaFieldKind::Str("1.0.0".into()),
							span: Span::DUMMY,
						},
						MetaField {
							ident: Ident {
								symbol: sym!("name"),
								span: Span::DUMMY,
							},
							value: MetaFieldKind::Vec(vec![
								MetaFieldKind::Str("https://paradigm.lighton.ai/api/v1".into()),
								MetaFieldKind::Str(
									"https://paradigm-preprod.lighton.ai/api/v1".into(),
								),
								MetaFieldKind::Str("https://paradigm-dev.lighton.ai/api/v1".into()),
							]),
							span: Span::DUMMY,
						},
					],
				}),
				ident: Ident {
					symbol: sym!(""),
					span: Span::DUMMY,
				},
				id: NodeId::DUMMY,
				span: Span::DUMMY,
			}),
			P::<Item>::new(Item {
				attrs: thin_vec![Attribute {
					kind: AttrKind::DocComment(sym!(" Imports the `builder.dapi` file")),
					style: AttrStyle::OuterOrInline,
					id: AttrId::make_id(),
					span: Span::DUMMY,
				}],
				ident: Ident {
					symbol: sym!("builder"),
					span: Span::DUMMY,
				},
				kind: ItemKind::Scope(ScopeKind::Unloaded),
				id: NodeId::DUMMY,
				span: Span::DUMMY,
			}),
			P::<Item>::new(Item {
				attrs: thin_vec![],
				ident: Ident {
					symbol: sym!("dashboard"),
					span: Span::DUMMY,
				},
				kind: ItemKind::Scope(ScopeKind::Loaded {
					inline: true,
					items: vec![P::<Item>::new(Item {
						attrs: thin_vec![],
						ident: Ident {
							symbol: sym!("dashboard"),
							span: Span::DUMMY,
						},
						kind: ItemKind::Path(Path {
							path: PathKind::String(Ident {
								symbol: sym!("dashboard"),
								span: Span::DUMMY,
							}),
							items: vec![Item {
								attrs: thin_vec![],
								ident: Ident {
									symbol: sym!(""),
									span: Span::DUMMY,
								},
								kind: ItemKind::Headers(Headers {
									headers: vec![
										HeaderField {
											attrs: vec![
												Attribute {
													kind: AttrKind::DocComment(sym!(" # Safety")),
													style: AttrStyle::OuterOrInline,
													id: AttrId::make_id(),
													span: Span::DUMMY,
												},
												Attribute {
													kind: AttrKind::DocComment(sym!(
														" This is a comment"
													)),
													style: AttrStyle::OuterOrInline,
													id: AttrId::make_id(),
													span: Span::DUMMY,
												},
												Attribute {
													kind: AttrKind::DocComment(sym!(
														" This is a second line of comment"
													)),
													style: AttrStyle::OuterOrInline,
													id: AttrId::make_id(),
													span: Span::DUMMY,
												},
												// @description("The API Key of the User")
												Attribute {
													kind: AttrKind::Normal(NormalAttr {
														item: AttrItem {},
													}),
													style: AttrStyle::OuterOrInline,
													id: AttrId::make_id(),
													span: Span::DUMMY,
												},
												// @prefix("Api-Key")
												Attribute {
													kind: AttrKind::Normal(NormalAttr {
														item: AttrItem {},
													}),
													style: AttrStyle::OuterOrInline,
													id: AttrId::make_id(),
													span: Span::DUMMY,
												},
											],
											ident: Ident {
												symbol: sym!("Authorization"),
												span: Span::DUMMY,
											},
											span: Span::DUMMY,
										},
										HeaderField {
											// @description("The Model of the User")
											attrs: vec![Attribute {
												kind: AttrKind::Normal(NormalAttr {
													item: AttrItem {},
												}),
												style: AttrStyle::OuterOrInline,
												id: AttrId::make_id(),
												span: Span::DUMMY,
											}],
											ident: Ident {
												symbol: sym!("X-Model"),
												span: Span::DUMMY,
											},
											span: Span::DUMMY,
										},
									],
								}),
								id: NodeId::DUMMY,
								span: Span::DUMMY,
							}],
						}),
						id: NodeId::DUMMY,
						span: Span::DUMMY,
					})],
					span: Span::DUMMY,
				}),
				id: NodeId::DUMMY,
				span: Span::DUMMY,
			}),
		],
		id: NodeId::ROOT,
		span: Span::DUMMY,
	}
}
