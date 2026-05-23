/**
 * SigmaLex — Indian Legal Procedure Guide
 * Sovereign JS: No external libraries (no jQuery, no React).
 * Self-contained sovereign data model.
 */

// ── Data Model ──────────────────────────────────────────────────────────────
const LEGAL_DATA = {
  civil: {
    title: "Civil Case Procedure",
    subtitle: "Code of Civil Procedure, 1908 (CPC)",
    steps: [
      {
        title: "File the Plaint",
        explanation: "Initiate proceedings by filing a plaint at the appropriate civil court with jurisdiction over the matter and cause of action.",
        statute: "Order VII CPC — Must state parties, facts, relief sought; court fee under Court Fees Act, 1870",
        caselaw: "Saleem Bhai v. State of Maharashtra (2003) — SC held that a plaint must be read as a whole to assess maintainability",
        tip: "Ensure proper valuation of the suit for court fees. Under-valuation can result in rejection under Order VII Rule 11 CPC."
      },
      {
        title: "Issue and Serve Summons",
        explanation: "Court issues summons to the defendant directing appearance or filing of a written statement within the stipulated time.",
        statute: "Order V CPC — Modes of service: personal, substituted, or by courier/electronic means (post 2002 amendment)",
        caselaw: "State of Madhya Pradesh v. Bhagirathi (1993) — Proper service is a fundamental condition for a valid decree",
        tip: "If the defendant evades service, apply for substituted service under Order V Rule 17 CPC without delay."
      },
      {
        title: "Written Statement by Defendant",
        explanation: "The defendant must file a written statement raising all defenses, including jurisdictional challenges and set-offs.",
        statute: "Order VIII CPC — 30 days initially, extendable to 90 days maximum. No further extension post-2002 amendment.",
        caselaw: "Kailash v. Nanhku (2005) — SC held courts have discretion to extend time, but not beyond 90 days",
        tip: "All defenses not raised in the written statement are deemed waived. Include all pleas, including res judicata and limitation."
      },
      {
        title: "Framing of Issues",
        explanation: "The court frames issues (points of fact and law) in dispute that will form the basis of trial and evidence.",
        statute: "Order XIV CPC — Issues may be of fact, law, or mixed. Court has inherent power to frame additional issues at any stage.",
        caselaw: "Ramesh Chand Ardawatiya v. Anil Panjwani (2003) — Issues should precisely define the controversy",
        tip: "Request the court to frame specific issues if any are vague. Poorly framed issues weaken the strength of evidence adduced."
      },
      {
        title: "Discovery, Inspection & Admission",
        explanation: "Parties exchange and inspect documents; admissions of facts reduce trial time and are binding.",
        statute: "Order XI, XII, XIII CPC — Discovery by interrogatories, document inspection, and requests for admission",
        caselaw: "Inacio Martins v. Narayan Hari Naik (1993) — Party can be compelled to produce documents relevant to the controversy",
        tip: "Issue interrogatories strategically. Admissions obtained here are conclusive and cannot be contradicted at trial."
      },
      {
        title: "Trial and Recording of Evidence",
        explanation: "Parties examine witnesses in chief, followed by cross-examination. Documentary evidence is proved and exhibited.",
        statute: "Indian Evidence Act, 1872 (IEA) — Sections 59-90 govern oral and documentary evidence; Order XVIII CPC governs trial procedure",
        caselaw: "Mangilal v. Suganchand Rathi (2003) — Cross-examination must challenge specific facts, not merely suggestion",
        tip: "Mark all documents with exhibit numbers. Failure to prove documents results in their exclusion from evidence."
      },
      {
        title: "Final Arguments",
        explanation: "Both parties make oral or written submissions to the court summarizing their case, evidence, and applicable law.",
        statute: "Order XVIII Rule 2 CPC — Plaintiff opens; defendant replies; plaintiff may rejoin on new matters only",
        caselaw: "Ram Shankar v. State of UP (1956) — Arguments should address each issue framed; unsupported arguments are disregarded",
        tip: "Prepare a written argument memorandum. Courts increasingly rely on written submissions in complex matters."
      },
      {
        title: "Judgment and Decree",
        explanation: "Court delivers judgment, which is then followed by a formal decree. The decree is the operative document for enforcement.",
        statute: "Order XX CPC — Judgment must contain facts, points for determination, decision and reasons; decree follows judgment",
        caselaw: "Surya Dev Rai v. Ram Chander Rai (2003) — A decree must conform to the judgment; divergence is an error",
        tip: "Apply for a certified copy of the decree immediately. The limitation period for execution runs from the decree date."
      },
      {
        title: "Execution of Decree",
        explanation: "The decree-holder files an application to execute the decree — attach property, arrest judgment-debtor, or seek delivery of property.",
        statute: "Order XXI CPC — Multiple modes: arrest, attachment, sale of property, receiver; application under Section 36 CPC",
        caselaw: "Jugalkishore Saraf v. Raw Cotton Co. Ltd (1955) — Execution court cannot go behind the decree",
        tip: "File execution within 12 years of the decree (Limitation Act, Art. 136). Identify and locate assets before filing."
      },
      {
        title: "Appeal",
        explanation: "An aggrieved party may appeal to a higher court on grounds of fact and law. Second appeal lies on substantial questions of law only.",
        statute: "Section 96 CPC (First Appeal); Section 100 CPC (Second Appeal to HC on substantial question of law); Order XLI CPC",
        caselaw: "Santosh Hazari v. Purushottam Tiwari (2001) — SC clarified 'substantial question of law' under Section 100 CPC",
        tip: "File the appeal within 30 days (district court) or 90 days (High Court) from the decree. Apply for stay of execution pending appeal."
      }
    ]
  },

  criminal: {
    title: "Criminal Case Procedure",
    subtitle: "Code of Criminal Procedure, 1973 (CrPC) & Indian Penal Code, 1860 (IPC)",
    steps: [
      {
        title: "Lodge FIR / Complaint",
        explanation: "A First Information Report (FIR) is filed at the police station for cognizable offences. For non-cognizable offences, a complaint is made to a magistrate.",
        statute: "Section 154 CrPC — FIR for cognizable offences; Section 155 CrPC — Non-cognizable; Section 190 CrPC — Magistrate may take cognizance",
        caselaw: "Lalita Kumari v. State of UP (2013) — SC held registration of FIR mandatory for cognizable offences",
        tip: "If police refuse to register FIR, send it by post to SP under Section 154(3) CrPC, or approach Magistrate under Section 156(3) CrPC."
      },
      {
        title: "Police Investigation",
        explanation: "Police investigate the offence — examine witnesses, collect evidence, conduct searches and seizures, and may arrest the accused.",
        statute: "Section 157 CrPC — Duty to investigate; Section 41 CrPC — Arrest without warrant; Section 165 CrPC — Search by police",
        caselaw: "D.K. Basu v. State of West Bengal (1997) — Landmark guidelines on arrest procedure and rights of arrestee",
        tip: "Any person arrested must be informed of grounds of arrest and right to bail. Insist on a memo of arrest with time of arrest."
      },
      {
        title: "Bail Application",
        explanation: "The accused may apply for bail — regular bail for bailable offences or anticipatory bail before arrest.",
        statute: "Section 436 CrPC (Bailable); Section 437 CrPC (Non-bailable before magistrate); Section 438 CrPC (Anticipatory bail); Section 439 (Sessions/HC)",
        caselaw: "Gurbaksh Singh Sibbia v. State of Punjab (1980) — Broad principles for anticipatory bail; Sushila Aggarwal v. State (2020) — No fixed term for AB",
        tip: "For heinous offences under NDPS, PMLA, UAPA — special provisions apply with stringent conditions for bail."
      },
      {
        title: "Charge Sheet & Cognizance",
        explanation: "Police file a charge sheet (challan) within 60 or 90 days. The Magistrate takes cognizance and issues process.",
        statute: "Section 173 CrPC — Police report/chargesheet; Section 190 CrPC — Taking of cognizance; Section 204 — Issue of process",
        caselaw: "State of Bihar v. J.A.C. Saldanha (1980) — Taking cognizance is application of judicial mind to the offence",
        tip: "If charge sheet is not filed within 60/90 days, the accused is entitled to statutory bail under the proviso to Section 167(2) CrPC."
      },
      {
        title: "Framing of Charges",
        explanation: "The Magistrate or Sessions Judge frames charges against the accused, who pleads guilty or not guilty.",
        statute: "Section 228 CrPC — Sessions trial; Section 240 CrPC — Warrant case by police; Section 251 CrPC — Summons case",
        caselaw: "State v. Shyam Sunder Trivedi (1995) — At charge stage, court looks for prima facie material, not proof beyond doubt",
        tip: "At this stage, file an application to discharge under Section 227 CrPC if the charge is groundless. Grounds include lack of evidence."
      },
      {
        title: "Trial and Evidence",
        explanation: "Prosecution leads evidence first, followed by the defence. Witnesses are examined in chief and cross-examined.",
        statute: "Chapter XVIII CrPC — Trial procedure; Indian Evidence Act, 1872; Section 313 CrPC — Examination of accused",
        caselaw: "Sharad Birdhichand Sarda v. State of Maharashtra (1984) — Golden rule for circumstantial evidence cases",
        tip: "Section 313 CrPC examination of the accused cannot be used as substantive evidence against them. Ensure thorough cross-examination."
      },
      {
        title: "Judgment: Conviction or Acquittal",
        explanation: "After arguments, the court pronounces judgment. If convicted, the sentence is passed after hearing the accused on sentencing.",
        statute: "Section 353 CrPC — Judgment; Section 354 — Language and contents; Section 360 — Probation; Section 361 — Special reasons for not using probation",
        caselaw: "Bachan Singh v. State of Punjab (1980) — 'Rarest of rare' doctrine for death penalty; sentencing must be proportionate",
        tip: "On conviction, immediately seek suspension of sentence and bail pending appeal. The right to appeal is a statutory right, not discretionary."
      },
      {
        title: "Appeal & Revision",
        explanation: "An accused may appeal to the Sessions Court or High Court. The High Court's revisionary jurisdiction corrects legal errors.",
        statute: "Section 374 CrPC — Appeal against conviction; Section 377 — Appeal against inadequate sentence; Section 397 — Revision",
        caselaw: "Krishnan v. State of Kerala (2006) — Appellate court must re-appreciate evidence and not merely rubber-stamp trial court",
        tip: "An appeal against acquittal by the State requires leave of the High Court and is more difficult to maintain than a conviction appeal."
      }
    ]
  },

  labour: {
    title: "Labour Dispute Procedure",
    subtitle: "Industrial Disputes Act, 1947 & Four Labour Codes (2019-2020)",
    steps: [
      {
        title: "Raise Industrial Dispute",
        explanation: "The aggrieved workman or trade union raises a dispute regarding terms of employment, discharge, dismissal, or retrenchment.",
        statute: "Section 2(k) & Section 10 Industrial Disputes Act, 1947 (IDA) — Conciliation, Boards, Labour Courts, Industrial Tribunal",
        caselaw: "Workmen of Dimakuchi Tea Estate v. Management (1958) — Defined 'workman' and scope of industrial dispute broadly",
        tip: "Raise the dispute within 3 years (limitation). Prefer conciliation proceedings first as they are faster and preserve the employment relationship."
      },
      {
        title: "Conciliation Proceedings",
        explanation: "The Conciliation Officer attempts to bring about a settlement between the parties through mediation.",
        statute: "Section 12 IDA — Conciliation procedure; Section 12(6) — Failure report if no settlement",
        caselaw: "Ram Avtar Sharma v. State of Haryana (1985) — Conciliation is mandatory; failure report must be proper and reasoned",
        tip: "A settlement reached during conciliation is binding under Section 18(3) IDA. Do not sign without fully understanding the terms."
      },
      {
        title: "Reference to Labour Court or Tribunal",
        explanation: "On failure of conciliation, the government refers the dispute to a Labour Court or Industrial Tribunal for adjudication.",
        statute: "Section 10 IDA — Government may refer; Schedule II — Labour Court jurisdiction; Schedule III — Tribunal jurisdiction",
        caselaw: "State of Bombay v. Hospital Mazdoor Sabha (1960) — Government has discretion to refer; cannot be compelled except by mandamus",
        tip: "If the government refuses to refer, file a writ of mandamus before the High Court challenging the refusal."
      },
      {
        title: "Pleadings and Evidence Before Labour Court",
        explanation: "Parties file statements of claims and written statements. Oral and documentary evidence is led.",
        statute: "Section 11 IDA — Powers of Labour Court; Industrial Disputes (Central) Rules — Procedure for hearings",
        caselaw: "Bharat Forge Co. v. A.B. Ibrahim (1962) — Labour Court must follow principles of natural justice",
        tip: "Gather all HR communications, appointment letters, PF statements, and payslips. These are critical evidence in termination disputes."
      },
      {
        title: "Award by Labour Court/Tribunal",
        explanation: "The Labour Court/Tribunal passes an award which is binding on both parties.",
        statute: "Section 17 IDA — Publication of award; Section 17A — Commencement and enforceability; Section 17B — Payment of wages during pendency of appeal",
        caselaw: "Hindustan Tin Works v. Employees (1979) — Reinstatement with full back wages is the rule for wrongful dismissal",
        tip: "An award becomes enforceable 30 days after publication unless challenged. Section 17B entitlement to wages pending High Court challenge."
      },
      {
        title: "Appeal / Writ Petition",
        explanation: "An award can be challenged by a writ petition under Article 226/227 of the Constitution before the High Court.",
        statute: "Article 226 Constitution of India — Writ jurisdiction; Section 34 Arbitration Act if award is under private arbitration",
        caselaw: "Bharat Sanchar Nigam Ltd. v. Man Singh (2012) — HC writ jurisdiction is supervisory, not appellate; limited interference",
        tip: "Grounds for interference are jurisdictional error, violation of natural justice, or perversity — not mere reappreciation of evidence."
      }
    ]
  },

  constitutional: {
    title: "Constitutional / Writ Petition Procedure",
    subtitle: "Constitution of India, 1950 — Articles 32 & 226",
    steps: [
      {
        title: "Establish Violation of Fundamental Right",
        explanation: "Identify which Fundamental Right (Part III) has been violated by State action. Only State action can be challenged under Articles 32/226.",
        statute: "Articles 12-35 Constitution of India — Fundamental Rights; Article 12 — Definition of 'State' includes government, instrumentalities",
        caselaw: "Maneka Gandhi v. Union of India (1978) — Expanded reading of Article 21 to include all facets of life and liberty",
        tip: "For private bodies, establish they are 'State' under the expanded test of Ajay Hasia v. Khalid Mujib (1981) — pervasive control test."
      },
      {
        title: "Choose Appropriate Writ",
        explanation: "Select the correct writ — Habeas Corpus, Mandamus, Prohibition, Certiorari, or Quo Warranto — based on the relief sought.",
        statute: "Article 32 — SC original jurisdiction; Article 226 — HC broader jurisdiction including non-fundamental rights and statutory rights",
        caselaw: "Dwarkanath v. ITO (1965) — HC writ jurisdiction is wider than SC; Bandhua Mukti Morcha v. UoI (1984) — PIL and habeas corpus",
        tip: "Approach HC under Article 226 first. SC under Article 32 is available but treated as a last resort given HC's concurrent jurisdiction."
      },
      {
        title: "Filing the Writ Petition",
        explanation: "Draft and file the petition with a synopsis, brief facts, grounds of challenge, interim relief sought, and supporting affidavit.",
        statute: "High Court (Writ) Rules of respective state; Rule 226 requires petitioner's affidavit verifying facts; certified copies of impugned order",
        caselaw: "S.P. Gupta v. Union of India (1981) — Liberalized locus standi for PILs; any public-spirited person may file",
        tip: "Obtain certified copies of all impugned orders before filing. Annexure deficiencies are a common ground for rejection at the admission stage."
      },
      {
        title: "Admission Hearing & Interim Relief",
        explanation: "The court hears the petition for admission. If admitted, it may issue a rule nisi or grant interim stay/status quo.",
        statute: "Order XXXIX Rules 1-2 CPC (as applicable) — Interim injunctions; Article 226(3) — Ex-parte interim orders",
        caselaw: "American Cyanamid v. Ethicon (applied in India) — Three-pronged test: prima facie case, balance of convenience, irreparable harm",
        tip: "The burden for interim relief is lower than final relief. Demonstrate urgency — many matters are won or lost at the interim stage."
      },
      {
        title: "Counter Affidavit by Respondent",
        explanation: "The State or other respondents file counter affidavits responding to the averments in the writ petition.",
        statute: "Rules of Court — Counter affidavit must be filed within time stipulated by court; rejoinder may be allowed at court's discretion",
        caselaw: "State of UP v. Singhara Singh (1964) — Acts done in contravention of statutory procedure are void and must be so stated",
        tip: "File a rejoinder to the counter affidavit to rebut new facts. Courts give weight to unrebutted averments in counter affidavits."
      },
      {
        title: "Final Hearing",
        explanation: "Detailed arguments on the merits of the constitutional challenge, including questions of vires (validity) and proportionality.",
        statute: "Full Bench/Division Bench hears substantial constitutional questions; Article 145(3) — SC Constitution Bench for substantial questions",
        caselaw: "K.S. Puttaswamy v. Union of India (2017) — 9-Judge bench on right to privacy; proportionality as constitutional standard",
        tip: "Lead with constitutional provisions, then statutes, then precedents. Courts are strict on constitutional arguments — cite the exact provision."
      },
      {
        title: "Judgment and Directions",
        explanation: "Court passes judgment, quashing impugned orders, or issuing mandamus to the State to act in a particular manner.",
        statute: "Article 142 — SC may pass orders necessary for complete justice; Article 144 — All authorities to act in aid of SC",
        caselaw: "Vineet Narain v. Union of India (1998) — SC issued continuing mandamus with monitoring; landmark for enforcement jurisprudence",
        tip: "Ensure directions in the judgment are specific and measurable. Vague directions lead to non-compliance and endless contempt proceedings."
      },
      {
        title: "Contempt Proceedings (if needed)",
        explanation: "If the order is not complied with, file a contempt petition to enforce the judgment.",
        statute: "Contempt of Courts Act, 1971 — Civil contempt (willful disobedience) and Criminal contempt; Article 215 — HC is a court of record",
        caselaw: "Prithipal Singh v. Union of India (2012) — Willful disobedience must be shown; inability to comply is a defense",
        tip: "Contempt is a weapon of last resort. First, issue a formal demand letter calling for compliance with a reasonable time before filing."
      }
    ]
  }
};

// ── Rendering Engine ────────────────────────────────────────────────────────
let currentType = 'civil';

function selectCase(type) {
  currentType = type;

  // Update active card
  const cards = document.querySelectorAll('.case-card');
  cards.forEach(c => {
    c.classList.toggle('active', c.dataset.type === type);
  });

  renderTimeline(type);
}

function renderTimeline(type) {
  const data = LEGAL_DATA[type];
  const container = document.getElementById('timeline-container');
  const title     = document.getElementById('timeline-title');
  const subtitle  = document.getElementById('timeline-subtitle');

  title.textContent    = data.title;
  subtitle.textContent = data.subtitle;

  container.innerHTML = '';

  data.steps.forEach(function(step, idx) {
    const card = document.createElement('div');
    card.className = 'step-card';
    card.style.setProperty('--idx', idx);

    const numStr = (idx + 1 < 10 ? '0' : '') + (idx + 1);

    card.innerHTML =
      '<div class="step-num-col">' +
        '<div class="step-num">' + numStr + '</div>' +
      '</div>' +
      '<div class="step-body">' +
        '<div class="step-title">' + step.title + '</div>' +
        '<div class="step-explanation">' + step.explanation + '</div>' +
        '<div class="step-meta">' +
          '<div class="meta-row">' +
            '<span class="meta-badge badge-statute">Statute</span>' +
            '<span class="meta-text">' + step.statute + '</span>' +
          '</div>' +
          '<div class="meta-row">' +
            '<span class="meta-badge badge-caselaw">Case Law</span>' +
            '<span class="meta-text">' + step.caselaw + '</span>' +
          '</div>' +
          '<div class="meta-row">' +
            '<span class="meta-badge badge-tip">Tip</span>' +
            '<span class="meta-text">' + step.tip + '</span>' +
          '</div>' +
        '</div>' +
      '</div>';

    container.appendChild(card);
  });
}

// ── Init ────────────────────────────────────────────────────────────────────
document.addEventListener('DOMContentLoaded', function() {
  renderTimeline('civil');
});
